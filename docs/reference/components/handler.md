# `Handler`

`Handler` and `HandlerRef` are the most general members of the [handler family](../../concepts/handlers.md): asynchronous, fallible components that transform an `Input` into an `Output` under a phantom `Code` tag, returning a `Result` against the context's abstract error type.

## Purpose

`Handler` exists for the computations that need everything the family offers — to run asynchronously *and* to be able to fail. A handler that calls a remote service must await the response and must report failures, so it occupies the corner of the family that is both async and fallible. Every other handler component is a special case obtained by dropping one of these capabilities: drop the failure path and a `Handler` becomes an [`AsyncComputer`](computer.md), drop the asynchrony and it becomes a [`TryComputer`](try_computer.md), drop both and it becomes a [`Computer`](computer.md). `Handler` is therefore the component a generic consumer bounds against when it wants to accept *any* computation regardless of which capabilities the underlying provider actually uses, because every simpler provider can be promoted up to a `Handler`.

This generality is why the family promotes toward `Handler` rather than away from it. A pure synchronous computer can serve as a handler that neither awaits nor errors; a fallible computer can serve as a handler that awaits trivially; an async computer can serve as a handler that never errors. Each of these is a safe widening, and the combinators perform them automatically, so a provider author writes against the weakest variant that fits and the wiring lifts it to `Handler` wherever a handler is required. The reverse — using a `Handler` where only a `Computer` is wanted — is not possible, since a general computation cannot be assumed pure or synchronous.

## Definition

`Handler` is a CGP component defined with `#[cgp_component]` under `#[async_trait]`, and its consumer trait `CanHandle` supertraits `HasErrorType` so its method can return the context's abstract error:

```rust
#[async_trait]
#[cgp_component(Handler)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanHandle<Code, Input>: HasErrorType {
    type Output;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}
```

The consumer trait `CanHandle<Code, Input>` combines the async and fallible refinements of the base signature. Its `handle` method is declared `async` and returns `Result<Self::Output, Self::Error>`, so it is the async counterpart of `CanTryCompute` and the fallible counterpart of `CanComputeAsync`. The `#[async_trait]` attribute rewrites the `async fn` into a method returning `impl Future<Output = Result<Self::Output, Self::Error>>`, avoiding any boxed future. The component is wired through the generated `HandlerComponent` marker, its provider trait is `Handler<Context, Code, Input>` with the context moved into an explicit first parameter, and the two `#[derive_delegate(...)]` attributes generate dispatching providers keyed on `Code` and on `Input`. The `HasErrorType` supertrait supplies the `Self::Error` named in the result.

The by-reference sibling `HandlerRef` is identical except that it borrows its input. Its consumer trait `CanHandleRef` also supertraits `HasErrorType` and declares `async fn handle_ref(&self, _tag: PhantomData<Code>, input: &Input) -> Result<Self::Output, Self::Error>`, taking `&Input` where `CanHandle` takes `Input`.

## Implementations

A `Handler` provider is a zero-sized struct implementing the provider trait for a generic context with an error type, returning a future that resolves to `Result<Output, Context::Error>`. The crate's `ReturnInput` provider shows the minimal async-and-fallible shape — it awaits nothing and succeeds unconditionally:

```rust
impl<Context, Code, Input> Handler<Context, Code, Input> for ReturnInput
where
    Context: HasErrorType,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(input)
    }
}
```

Because `Handler` is the top of the promotion lattice, most `Handler` implementations are produced by promoting a simpler provider rather than written directly. A [`Computer`](computer.md) is promoted to `Handler` by wrapping its output in a non-awaiting future that returns `Ok` (`Promote`); a [`TryComputer`](try_computer.md) is promoted by wrapping its result in a non-awaiting future (`PromoteAsync`); an [`AsyncComputer`](computer.md) is promoted by wrapping its awaited output in `Ok` (`Promote`); and a `Computer` whose output is already a `Result` is promoted by unwrapping that result inside a future (`TryPromote`). The `PromoteRef` combinator additionally bridges between `Handler` and `HandlerRef` by dereferencing or re-borrowing the input. These promotions are the subject of [handler combinators](../providers/handler_combinators.md); a provider author rarely implements `Handler` by hand and instead lets the wiring lift the narrowest fitting variant.

## Examples

A generic consumer that bounds its context by `CanHandle` accepts any wired computation, whatever its underlying capabilities:

```rust
use core::marker::PhantomData;
use cgp::prelude::*;
use cgp::extra::handler::CanHandle;

async fn run_with<Context, Code>(
    context: &Context,
    input: String,
) -> Result<Context::Output, Context::Error>
where
    Context: CanHandle<Code, String>,
{
    context.handle(PhantomData::<Code>, input).await
}
```

The function `run_with` works for any context that wires a handler for the given `Code` and `String` input, whether the wired provider is a pure `Computer`, a fallible `TryComputer`, an `AsyncComputer`, or a genuine `Handler` — the promotion combinators make each of them satisfy `CanHandle`. This is why generic pipeline code targets `Handler`: it is the one bound every member of the family can meet. In practice the [`#[cgp_computer]`](../macros/cgp_computer.md) and [`#[cgp_producer]`](../macros/cgp_producer.md) macros wire the promotion table so that a function written as a simple computer or producer answers `CanHandle` automatically, which is what lets such a consumer call it.

## Related constructs

`Handler` is the general corner of the [handler family](../../concepts/handlers.md), generalizing [`Computer`](computer.md) (drop fallibility and asynchrony), [`AsyncComputer`](computer.md) (drop fallibility), and [`TryComputer`](try_computer.md) (drop asynchrony). It supertraits [`HasErrorType`](has_error_type.md), which supplies the `Self::Error` it returns. The combinators that promote the simpler variants up to `Handler`, and that bridge `Handler` with `HandlerRef`, are documented in [handler combinators](../providers/handler_combinators.md), and chaining handlers into pipelines is covered in [monadic handlers](../../concepts/monadic-handlers.md). The no-input member of the family is [`Producer`](producer.md). Dispatching a handler on its `Code` or `Input` uses [`UseDelegate`](../providers/use_delegate.md) and the family's `UseInputDelegate`, per [dispatching](../../concepts/dispatching.md).

## Source

`Handler` and `HandlerRef` are defined in [crates/extra/cgp-handler/src/components/handler.rs](../../../crates/extra/cgp-handler/src/components/handler.rs). The `ReturnInput` provider is in [crates/extra/cgp-handler/src/providers/return_input.rs](../../../crates/extra/cgp-handler/src/providers/return_input.rs), and the promotion combinators that lift simpler providers into `Handler` are in [crates/extra/cgp-handler/src/providers/](../../../crates/extra/cgp-handler/src/). The components are re-exported through `cgp::extra::handler`. Behavioral tests exercising handlers and their promotions are in [crates/tests/cgp-tests/tests/handler_tests/handler_macro.rs](../../../crates/tests/cgp-tests/tests/handler_tests/handler_macro.rs).
