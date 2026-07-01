# Monad providers

The monad providers turn a list of handlers and a choice of monad into a single composed handler that short-circuits on the appropriate branch: `PipeMonadic` is the pipeline builder, the `IdentMonadic` / `OkMonadic` / `ErrMonadic` markers (with their transformer forms) name the monad, and `BindOk` / `BindErr` are the per-step bind providers that implement the branching.

## Purpose

These providers implement [monadic handler composition](../../concepts/monadic-handlers.md) on top of the [`Computer`](../components/computer.md) family. They exist so that a sequence of handlers whose outputs carry a "continue" case and a "stop" case can be chained without manually pattern-matching each step: the monad decides which case threads forward and which short-circuits, and the providers assemble the composition in types. The result of building a pipeline is itself a provider for `Computer`, `AsyncComputer`, `TryComputer`, and `Handler`, so a monadic pipeline wires into a context exactly like any other handler provider.

The providers divide into three groups. `PipeMonadic` is the entry point a user wires or invokes. The monad markers are the zero-sized types that select the short-circuiting behavior. The bind providers are the lower-level building blocks that `PipeMonadic` composes internally, and that can also be used directly with the non-monadic [`PipeHandlers`](../providers/handler_combinators.md) when finer control is wanted.

## The pipeline provider

`PipeMonadic<M, Providers>` is the provider that composes a handler list `Providers` under a monad `M` into a single short-circuiting handler:

```rust
pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);
```

`M` is a monad marker and `Providers` is a [type-level list](../types/cons.md) of handler providers, written with `Product![...]`. `PipeMonadic` implements the handler components by folding the list: it delegates `ComputerComponent` and `AsyncComputerComponent` to the provider that an internal `BindProviders<M>` computation produces from the list. That fold walks the list so that the first provider runs on the input and its result is bound, via the monad, to the monadically-composed rest of the list — each step running only on the previous step's continue branch.

`PipeMonadic` also implements `TryComputerComponent` and `HandlerComponent`, the fallible and async-fallible handler components, by a bridge through the err monad. For these it first maps every provider in the list to `TryPromote<Provider>` (demoting fallible handlers to plain `Computer`s whose output is an explicit `Result`), applies `ErrMonadic` as a transformer on top of the given monad `M`, composes that demoted list under the transformed monad, and finally wraps the composed provider back in `TryPromote` to restore the fallible interface. The effect is that a `PipeMonadic` over fallible handlers short-circuits on the context's error type in addition to whatever branching `M` itself contributes.

## Monad markers

The monad markers are zero-sized types that select which branch of a step's output continues the pipeline and which short-circuits. CGP defines three base markers and a transformer form for the two that branch on `Result`.

`IdentMonadic` is the identity monad: it threads every value forward and never short-circuits, so a `PipeMonadic<IdentMonadic, ...>` is equivalent to plain composition with `PipeHandlers`.

```rust
pub struct IdentMonadic;
```

`OkMonadic` short-circuits on `Ok` and continues on `Err`, and `ErrMonadic` short-circuits on `Err` and continues on `Ok`:

```rust
pub struct OkMonadic;
pub struct ErrMonadic;
```

`ErrMonadic` is the familiar early-return-on-error behavior, where the first `Err` produced by any step becomes the pipeline's output and the rest do not run. `OkMonadic` is its mirror, stopping at the first `Ok`.

Each `Result`-branching marker has a transformer form, `OkMonadicTrans<M>` and `ErrMonadicTrans<M>`, that applies its behavior on top of a base monad `M` so monads can stack over nested result types:

```rust
pub struct OkMonadicTrans<M>(pub PhantomData<M>);
pub struct ErrMonadicTrans<M>(pub PhantomData<M>);
```

Writing `OkMonadicTrans<ErrMonadic>` builds a monad that short-circuits on an outer `Ok` while threading an inner `Result` through the err monad beneath it. The bare `OkMonadic` and `ErrMonadic` markers produce their own transformer forms over `IdentMonadic` when used as transformers, so a single layer of branching needs no explicit transformer.

## Bind providers

`BindOk` and `BindErr` are the per-step providers that implement a single bind of the ok and err monads. `PipeMonadic` composes them internally, but they are also usable directly as handler providers — for example inside a [`PipeHandlers`](../providers/handler_combinators.md) list — when a pipeline is built step by step rather than through `PipeMonadic`.

```rust
pub struct BindOk<M, Cont>(pub PhantomData<(M, Cont)>);
pub struct BindErr<M, Cont>(pub PhantomData<(M, Cont)>);
```

In both, `M` is the monad layer beneath this bind and `Cont` is the continuation provider to run on the continue branch. `BindErr<M, Cont>` implements `Computer` and `AsyncComputer` for an input of `Result<T1, E>`: on `Ok(value)` it runs `Cont` on `value` and lifts the continuation's output back through `M`, and on `Err(err)` it short-circuits by lifting the error directly to the output, skipping `Cont`. `BindOk<M, Cont>` is the mirror, branching on `Result<T, E1>`: it runs `Cont` on the `Err` payload and short-circuits on `Ok`. The `M` parameter is what lets these binds nest — at the bottom of a single-layer pipeline it is `IdentMonadic`, and a stacked monad threads a deeper monad through it.

## The `TryPromoteProviders` mapper

`TryPromoteProviders` is the type-level mapper that `PipeMonadic` uses to demote a whole list of fallible handler providers to infallible ones in one step:

```rust
pub struct TryPromoteProviders;

impl MapType for TryPromoteProviders {
    type Map<Provider> = TryPromote<Provider>;
}
```

It implements [`MapType`](../traits/map_type.md) by mapping each provider to `TryPromote<Provider>`, so applying it to a handler list with `MapFields` rewrites every element to its `TryPromote` form. `PipeMonadic` uses this when implementing the fallible handler components, turning a list of `TryComputer` providers into a list of plain `Computer` providers whose output is an explicit `Result` before composing them under the err-transformed monad. The `TryPromote` provider it wraps each element in is documented with the [handler combinators](../providers/handler_combinators.md); it converts between the fallible and infallible handler interfaces in both directions.

## Examples

The simplest use composes a homogeneous list under a base monad. With an `Increment` computer that returns `Result<u8, &str>` — `Ok` on success and `Err("overflow")` on overflow — composing three under `ErrMonadic` chains on the `Ok` value and stops at the first error:

```rust
PipeMonadic::<ErrMonadic, Product![Increment, Increment, Increment]>::compute(&context, code, 253)
// 253 -> Ok(254) -> Ok(255) -> Err("overflow")
```

A single bind step can be assembled by hand and run through `PipeHandlers`, which is what `PipeMonadic` does internally for a two-element list:

```rust
PipeHandlers::<Product![Increment, BindErr<IdentMonadic, Increment>]>::compute(&context, code, 1)
// 1 -> Ok(2) -> BindErr runs the second Increment on 2 -> Ok(3)
```

Stacking monads handles nested results. Composing handlers that return `Result<Result<(), u8>, &str>` under `OkMonadicTrans<ErrMonadic>` short-circuits on the outer `Ok` while threading the inner `Result` through the err monad, and the same list composed under `OkMonadic` can be driven through the fallible `try_compute` and async `handle` entry points because `PipeMonadic` implements `TryComputer` and `Handler` as well:

```rust
PipeMonadic::<OkMonadic, Product![ReturnOkErr, ReturnOkOk, ReturnOkErr]>::try_compute(&context, code, 1)
```

## Related constructs

`PipeMonadic` generalizes the non-monadic [handler combinators](../providers/handler_combinators.md): `PipeHandlers` and `ComposeHandlers` chain handlers feeding each output straight into the next, which is exactly what `PipeMonadic<IdentMonadic, ...>` reduces to, while the `TryPromote` provider those combinators define is what `PipeMonadic` uses to bridge fallible and infallible handlers. The monads these providers consume are defined by the trait layer in [monad traits](../traits/monad.md) — `MonadicTrans`, `MonadicBind`, `ContainsValue`, and `LiftValue` — and the conceptual overview of why a monadic pipeline short-circuits is in [monadic handlers](../../concepts/monadic-handlers.md). The pipelines build providers for the [`Computer`](../components/computer.md) family, and `TryPromoteProviders` relies on [`MapType`](../traits/map_type.md) and `MapFields` to map over the handler list. For selecting one handler among several by a type-level key rather than running them in sequence, see [dispatch combinators](../providers/dispatch_combinators.md).

## Source

The pipeline provider, `TryPromoteProviders`, and the internal `BindProviders` fold are in [crates/extra/cgp-monad/src/providers/pipe_monadic.rs](../../../crates/extra/cgp-monad/src/providers/pipe_monadic.rs). The monad markers and bind providers are in [crates/extra/cgp-monad/src/monadic/](../../../crates/extra/cgp-monad/src/monadic/) — `ident.rs` for `IdentMonadic`, `ok.rs` for `OkMonadic` / `OkMonadicTrans` / `BindOk`, and `err.rs` for `ErrMonadic` / `ErrMonadicTrans` / `BindErr`. Usage is exercised in [crates/tests/cgp-tests/tests/monadic_handlers/](../../../crates/tests/cgp-tests/tests/monadic_handlers/) (`ok.rs`, `err.rs`, `ok_err_trans.rs`).
