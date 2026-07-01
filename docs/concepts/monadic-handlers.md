# Monadic handlers

Monadic handlers are a way to chain CGP handlers into a pipeline that automatically short-circuits when an intermediate step produces a result the chain should stop on, threading a single `Output` value through the whole sequence.

## Purpose

The monad layer solves the problem of sequencing handlers when some of those handlers produce a result that should end the pipeline early. Plain handler composition with [`ComposeHandlers`](../reference/providers/handler_combinators.md) and `PipeHandlers` feeds the output of each provider straight into the next, which is the right behavior only when every step always wants to continue. The moment a step can yield a value that means "stop here, this is the final answer," straight composition is wrong: the later steps would run on a value they were never meant to see. The classic case is a `Result`, where an `Err` should abandon the rest of the pipeline and become the output directly, but the same shape appears whenever a step's output type carries two possibilities — one to keep going with, one to return immediately.

A monad captures exactly that branching. It describes, for a given output type, which case threads forward into the next handler and which case short-circuits out as the final result, plus how to lift a plain value back into that output type. With the monad chosen, a list of handlers can be composed so that each handler runs only on the "continue" branch of the previous one, and any "stop" value flows untouched to the end. The handler that the monad layer builds is itself an ordinary provider for the [`Computer`](../reference/components/computer.md) family, so a monadic pipeline plugs into the same wiring as any other handler.

CGP ships three monads. The *identity* monad threads every value forward and never short-circuits, recovering plain composition. The *ok* monad short-circuits on `Ok` and continues on `Err`. The *err* monad short-circuits on `Err` and continues on `Ok` — the familiar `?`-style early return where the first error wins. These can also be stacked, so that a pipeline over a nested `Result<Result<T, E>, F>` can short-circuit on the outer error while threading the inner result.

## Behavior

A monadic pipeline runs a list of handlers left to right, where each handler is invoked only on the branch of the previous handler's output that the monad designates as "continue." The first handler receives the pipeline's input. Its output is split by the monad: the continue case is fed as the input to the second handler, and the short-circuit case is lifted directly into the pipeline's final output type, skipping every remaining handler. This repeats down the list, so the pipeline's result is either the short-circuit value of whichever handler first produced one, or the output of the final handler if none short-circuited.

The err monad makes this concrete. Composing three handlers that each return `Result<T, E>` under the err monad runs the first handler on the input; if it returns `Ok`, the contained value is passed to the second handler, and so on; if any handler returns `Err`, that error becomes the pipeline's output immediately and the remaining handlers do not run. This is the type-level equivalent of writing each step with a `?` operator. The ok monad is the mirror image: it continues on `Err` and stops on `Ok`, which is useful for pipelines that retry or fall through until something succeeds.

The whole construction lives in types. The monad is a zero-sized marker, the handler list is a [type-level list](../reference/types/cons.md), and the pipeline provider that results carries no runtime value — it is assembled by the trait machinery at compile time, so chaining handlers monadically costs nothing at runtime beyond the branches the logic actually requires.

## Examples

The entry point is the [`PipeMonadic`](../reference/providers/monad_providers.md) provider, which takes a monad marker and a `Product!` list of handler providers. Given an `Increment` computer that returns `Result<u8, &str>` — incrementing on success and reporting `"overflow"` as the error — composing three copies under the err monad runs them in sequence and stops at the first overflow:

```rust
PipeMonadic::<ErrMonadic, Product![Increment, Increment, Increment]>::compute(&context, code, 253)
// 253 -> Ok(254) -> Ok(255) -> Err("overflow")
```

Each `Increment` runs on the `Ok` value of the one before it; the third increment overflows, so its `Err("overflow")` becomes the pipeline's output and no further step runs. Wiring the same list under [`IdentMonadic`](../reference/providers/monad_providers.md) instead would thread the `Result` value forward unchanged, never short-circuiting — which is plain composition and is rarely what a `Result`-producing chain wants.

Stacking monads handles nested results. A pipeline of handlers returning `Result<Result<(), u8>, &str>` can short-circuit on the outer `&str` error while threading the inner `Result<(), u8>` by composing under `OkMonadicTrans<ErrMonadic>` — the err monad applied as the base, transformed by the ok monad on top:

```rust
PipeMonadic::<OkMonadicTrans<ErrMonadic>, Product![ReturnOkErr, ReturnOkOk, ReturnOkErr]>
    ::compute(&context, code, 1)
```

Here an outer `Err` ends the pipeline immediately, while an outer `Ok` unwraps to the inner `Result` that the next handler continues from.

## Related constructs

Monadic handlers build directly on the handler family: the pipelines they produce are providers for [`Computer`](../reference/components/computer.md) and its async and fallible relatives, so the same providers that a monadic pipeline composes are the providers any handler wiring uses. [`PipeMonadic`](../reference/providers/monad_providers.md) is the user-facing provider, and the monad markers `IdentMonadic`, `OkMonadic`, and `ErrMonadic` — together with their transformer forms and the per-step `BindOk` / `BindErr` providers — are documented there. The trait layer that defines what a monad *is* (which branch continues, which short-circuits, and how to lift a value) lives in [the monad traits](../reference/traits/monad.md): `MonadicTrans`, `MonadicBind`, `LiftValue`, and `ContainsValue`. The non-monadic composition that monadic pipelines generalize — straight `ComposeHandlers` and `PipeHandlers`, plus the `TryPromote` bridge between fallible and infallible handlers — is covered in [handler combinators](../reference/providers/handler_combinators.md). For dispatching to different handlers by a type-level key rather than running them in sequence, see [dispatch combinators](../reference/providers/dispatch_combinators.md).

## Source

The monad layer lives in [crates/extra/cgp-monad/src/](../../crates/extra/cgp-monad/src/), re-exported as `cgp::extra::monad`. The pipeline provider is in `providers/pipe_monadic.rs`, the monad markers in `monadic/{ident,ok,err}.rs`, and the trait layer in `traits/`. The behavior described here is exercised by the tests in [crates/tests/cgp-tests/tests/monadic_handlers/](../../crates/tests/cgp-tests/tests/monadic_handlers/) (`ok_monadic.rs`, `err_monadic.rs`, `ok_err_monadic_trans.rs`).
