# `Producer`

`Producer` is the no-input member of the [handler family](../../concepts/handlers.md): a synchronous, infallible component that produces an `Output` from the context and a phantom `Code` tag alone, with no `Input`.

## Purpose

`Producer` exists for computations that take nothing to compute and simply yield a value. A handler that supplies a default configuration, a constant, or a value read entirely from the context has no `Input` to transform — it only needs the context and a `Code` tag to know which value to produce. The rest of the handler family threads an `Input` through every method; `Producer` is the degenerate case where that input is absent. It is the simplest component in the family: synchronous, infallible, and inputless, producing an `Output` directly.

Being inputless does not isolate `Producer` from the family — it makes it the natural source of values that flow into the other handlers. A producer can be promoted into any computer or handler by ignoring whatever input that variant supplies and returning the produced value regardless. This is exactly how a constant or a context-derived value enters a computation pipeline: it is produced once, independent of the input, and the promotion machinery adapts it to the input-taking shape the pipeline expects.

## Definition

`Producer` is a CGP component defined with `#[cgp_component]`, and its consumer trait `CanProduce` differs from the rest of the family only in carrying no `Input` parameter:

```rust
#[cgp_component(Producer)]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanProduce<Code> {
    type Output;

    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
```

The consumer trait `CanProduce<Code>` takes only a `Code` tag, not an `Input`. Its `produce` method takes `&self` (the context) and a `PhantomData<Code>` naming which value to produce, returning the associated `Output`. The component is wired through the generated `ProducerComponent` marker, and the macro generates the provider trait `Producer<Context, Code>` with the context moved into an explicit first parameter. Because there is no input to dispatch on, `Producer` carries only the single `#[derive_delegate(UseDelegate<Code>)]` attribute — dispatch is on the `Code` tag alone, with no `UseInputDelegate` counterpart. `Producer` does not supertrait `HasErrorType`: like `Computer`, it is infallible and synchronous, returning its `Output` directly rather than a `Result`.

## Implementations

A `Producer` provider is a zero-sized struct implementing the provider trait for a generic context, choosing the `Output` it yields. A producer that ignores the context and returns a constant has the minimal shape:

```rust
#[cgp_new_provider]
impl<Context, Code> Producer<Context, Code> for MagicNumber {
    type Output = u64;

    fn produce(_context: &Context, _code: PhantomData<Code>) -> u64 {
        42
    }
}
```

A producer is promoted into every other handler component by discarding the input that the target variant supplies and returning the produced value unchanged. The `Promote` combinator lifts a `Producer` into a [`Computer`](computer.md): the resulting computer takes an input, ignores it, and returns `Producer::produce(context, code)`. From there the rest of the promotion lattice carries the value up to the fallible, async, and by-reference variants. The bundled `PromoteProducer` table wires every other handler component to the appropriate promoter, so delegating a context's whole handler surface to `PromoteProducer<Self>` makes one producer answer `CanCompute`, `CanTryCompute`, `CanComputeAsync`, `CanHandle`, and their `Ref` forms. These combinators are documented in [handler combinators](../providers/handler_combinators.md).

## Examples

A producer wired into a context, then invoked both directly and as an input-ignoring computer, illustrates how it joins the rest of the family:

```rust
use core::marker::PhantomData;
use cgp::prelude::*;
use cgp::extra::handler::{CanProduce, Producer, ProducerComponent};

#[cgp_new_provider]
impl<Context, Code> Producer<Context, Code> for MagicNumber {
    type Output = u64;

    fn produce(_context: &Context, _code: PhantomData<Code>) -> u64 {
        42
    }
}

pub struct App;

delegate_components! {
    App {
        ProducerComponent: MagicNumber,
    }
}

fn run(app: &App) -> u64 {
    app.produce(PhantomData::<()>) // returns 42
}
```

Here `MagicNumber` produces `42` from the `Code` tag alone, and `App` delegates `ProducerComponent` to it, giving `App` the `CanProduce<(), Output = u64>` capability. The [`#[cgp_producer]`](../macros/cgp_producer.md) macro turns a plain zero-argument function such as `fn magic_number() -> u64 { 42 }` into exactly this provider, and additionally wires `PromoteProducer<Self>` so the same function also answers the input-taking components — `MagicNumber::compute(&app, code, &input)` then returns `42` while ignoring `input`.

## Related constructs

`Producer` is the no-input member of the [handler family](../../concepts/handlers.md), where the broader `Code`/`Input`/`Output` model and its axes are explained. Its input-taking counterparts are [`Computer`](computer.md) for the synchronous infallible case, [`TryComputer`](try_computer.md) for the fallible case, and [`Handler`](handler.md) for the general async-and-fallible case; a producer promotes into any of them by ignoring the supplied input. The combinators that perform those promotions, including the `Promote` provider and the `PromoteProducer` table, are documented in [handler combinators](../providers/handler_combinators.md). The macro that generates a producer from a zero-argument function is [`#[cgp_producer]`](../macros/cgp_producer.md), and dispatching a producer on its `Code` tag uses [`UseDelegate`](../providers/use_delegate.md), per [dispatching](../../concepts/dispatching.md).

## Source

`Producer` is defined in [crates/extra/cgp-handler/src/components/produce.rs](../../../crates/extra/cgp-handler/src/components/produce.rs). The `Promote` combinator that lifts it into a `Computer` is in [crates/extra/cgp-handler/src/providers/promote.rs](../../../crates/extra/cgp-handler/src/providers/promote.rs), and the `PromoteProducer` table in [crates/extra/cgp-handler/src/providers/promote_all.rs](../../../crates/extra/cgp-handler/src/providers/promote_all.rs). The component is re-exported through `cgp::extra::handler`. Behavioral tests exercising a producer and its promotions are in [crates/tests/cgp-tests/tests/handlers/producer_macro.rs](../../../crates/tests/cgp-tests/tests/handlers/producer_macro.rs).
