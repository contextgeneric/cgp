# Monad traits

The monad traits — `MonadicTrans`, `MonadicBind`, `LiftValue`, and `ContainsValue` — are the type-level interface that defines what a monad is for [monadic handler composition](../../concepts/monadic-handlers.md): which branch of an output value continues a pipeline, which branch short-circuits, and how values move between those branches.

## Purpose

These four traits factor a monad into the distinct capabilities the monadic pipeline machinery needs from it. A monad in CGP is a zero-sized marker type (`IdentMonadic`, `OkMonadic`, `ErrMonadic`, and their transformer forms), and these traits are what give that marker meaning. They are plain capability traits rather than CGP components — they have no generated provider trait or `…Component` marker and are not wired through `delegate_components!`; instead, the [`PipeMonadic`](../providers/monad_providers.md) provider and the per-step `BindOk` / `BindErr` providers consume them as ordinary trait bounds while building a pipeline at compile time.

The split exists because building a monadic pipeline requires three separable decisions. Composing a list of handlers under a monad needs to know how to turn a continuation provider into a bind step — that is `MonadicBind`. Each bind step needs to know, for a given output type, what the underlying value beneath the monad's wrapper is, and how to put a value back into that output type — that is `ContainsValue` and `LiftValue`. Stacking one monad on top of another needs to apply a monad as a transformer over a base monad — that is `MonadicTrans`. Keeping these as separate traits lets the same marker serve all three roles and lets monads stack by composing their implementations.

## Definition

`MonadicBind` maps a continuation provider to the provider that runs one bind step of the monad:

```rust
pub trait MonadicBind<Provider> {
    type Provider;
}
```

The `Provider` parameter is the continuation — the handler that should run on the monad's continue branch — and the `Provider` associated type is the bind provider that wraps it. For the base monads this resolves to a `BindOk` or `BindErr` provider; for `IdentMonadic` it is the continuation unchanged.

`MonadicTrans` applies a monad as a transformer onto a base monad:

```rust
pub trait MonadicTrans<M> {
    type M;
}
```

The `M` parameter is the base monad being transformed, and the `M` associated type is the resulting stacked monad. This is what lets `OkMonadic` be written as `OkMonadicTrans<ErrMonadic>` when a pipeline operates over a nested result type, layering the ok behavior on top of the err behavior.

`ContainsValue` reads, for a given monadic output type, the value type that sits beneath this monad's wrapper:

```rust
pub trait ContainsValue<Output> {
    type Value;
}
```

The `Output` parameter is the full output type a step produces; the `Value` associated type is the type carried in the branch the monad threads through, which a continuation handler consumes or which a deeper monad layer unwraps further.

`LiftValue` moves values into a step's output type, in two directions:

```rust
pub trait LiftValue<Value, Output> {
    type Output;

    fn lift_value(value: Value) -> Self::Output;

    fn lift_output(output: Output) -> Self::Output;
}
```

The associated `Output` is the final output type of the lift. `lift_value` takes a bare value from the continue or short-circuit branch and wraps it into that output type, and `lift_output` takes a value already in the inner `Output` shape and re-wraps it. The two methods correspond to the two branches a bind step takes: one lifts the short-circuit value back out as the result, the other forwards a continuation's already-computed output.

## Implementations

`IdentMonadic` implements all four traits as identities, which is what makes it thread every value forward without ever short-circuiting. `MonadicTrans<M>` returns `M` unchanged, `MonadicBind<Provider>` returns `Provider` unchanged, `ContainsValue<T>::Value` is `T`, and `LiftValue<T, T>` is the identity on `T` for both methods.

`OkMonadic` and `ErrMonadic` implement the traits to branch on a `Result`, in mirror image of each other. For `ErrMonadic`, `ContainsValue<Result<T, E>>::Value` is `T` — the `Ok` payload is the value threaded forward — and `LiftValue<T, Result<T, E>>::lift_value` is `Ok`, so the continue branch is `Ok` and an `Err` short-circuits. For `OkMonadic` the roles are swapped: `ContainsValue<Result<T, E>>::Value` is `E`, and `lift_value` is `Err`, so the continue branch is `Err` and an `Ok` short-circuits. The `MonadicBind` impl of each base monad produces the corresponding `BindOk` or `BindErr` step over `IdentMonadic`, and the `MonadicTrans` impl wraps the marker in its transformer form (`OkMonadicTrans`, `ErrMonadicTrans`).

The transformer forms `OkMonadicTrans<M>` and `ErrMonadicTrans<M>` implement the same traits by delegating one layer down to the base monad `M`. Their `ContainsValue` and `LiftValue` impls require `M: ContainsValue<V, Value = Result<…>>`, peeling their own `Result` layer and handing the rest to `M`; their `MonadicTrans` impl composes transformers so a stack like `OkMonadicTrans<ErrMonadic>` resolves layer by layer. This delegation is what allows monads to stack to arbitrary depth over nested result types.

## Related constructs

These traits are consumed by the monad providers in [monad providers](../providers/monad_providers.md): `PipeMonadic` uses `MonadicTrans` and `MonadicBind` to fold a handler list into a single pipeline provider, while `BindOk` and `BindErr` use `ContainsValue` and `LiftValue` in their `Computer` and `AsyncComputer` implementations to split and re-lift each step's output. The high-level picture of how the pieces fit — why a pipeline short-circuits and how the monads compose — is in [monadic handlers](../../concepts/monadic-handlers.md). The pipelines built from these traits implement the [`Computer`](../components/computer.md) family, so they slot into the same wiring as the [handler combinators](../providers/handler_combinators.md) `ComposeHandlers` and `PipeHandlers`, which compose handlers without the short-circuiting branch.

## Source

The traits are defined in [crates/extra/cgp-monad/src/traits/](../../../crates/extra/cgp-monad/src/traits/) — `monadic_trans.rs`, `bind.rs`, `lift.rs`, and `value.rs`. Their implementations for each monad marker are in [crates/extra/cgp-monad/src/monadic/](../../../crates/extra/cgp-monad/src/monadic/) (`ident.rs`, `ok.rs`, `err.rs`). The behavior is exercised by [crates/tests/cgp-tests/src/tests/monad/](../../../crates/tests/cgp-tests/src/tests/monad/).
