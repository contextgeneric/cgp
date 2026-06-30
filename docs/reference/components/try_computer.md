# `TryComputer`

`TryComputer` and `TryComputerRef` are the fallible synchronous corner of the [handler family](../../concepts/handlers.md): components that transform an `Input` into an `Output` under a phantom `Code` tag, returning a `Result` against the context's abstract error type.

## Purpose

`TryComputer` exists for synchronous computations that can fail. A computation that parses a string, looks up a key, or checks an invariant may not be able to produce its output, and it needs a way to report the failure. `TryComputer` gives it one: instead of returning `Output` directly like [`Computer`](computer.md), its method returns `Result<Output, Error>`, where the error is the context's shared abstract error type. This places it one step up from `Computer` on the fallibility axis of the family — still synchronous, but now able to fail — and one step below [`Handler`](handler.md), which adds asynchrony on top of fallibility.

Returning the *context's* abstract error rather than a concrete one is what keeps a `TryComputer` provider generic. A provider does not commit to `anyhow::Error` or `std::io::Error`; it returns `Self::Error`, and the concrete error type is decided once, at wiring time, by whichever error backend the context plugs in. This is why both fallible computer components supertrait [`HasErrorType`](has_error_type.md): the supertrait is what gives the trait a `Self::Error` to name in its `Result`, and it is what ties every fallible component in a context to the same error type so their results compose.

## Definition

`TryComputer` is a CGP component defined with `#[cgp_component]`, and its consumer trait `CanTryCompute` supertraits `HasErrorType` so that its method can return the context's abstract error:

```rust
#[cgp_component(TryComputer)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanTryCompute<Code, Input>: HasErrorType {
    type Output;

    fn try_compute(
        &self,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}
```

The consumer trait `CanTryCompute<Code, Input>` mirrors `CanCompute` but for the fallible case. Its `try_compute` method takes `&self`, a `PhantomData<Code>` naming the computation, and the `Input` by value, returning `Result<Self::Output, Self::Error>` — the associated `Output` on success and the context's abstract `Error` (supplied by the `HasErrorType` supertrait) on failure. The component is wired through the generated `TryComputerComponent` marker, and the macro generates the provider trait `TryComputer<Context, Code, Input>` with the context moved into an explicit first parameter. The two `#[derive_delegate(...)]` attributes generate dispatching providers keyed on `Code` and on `Input`.

The by-reference sibling `TryComputerRef` is identical except that it borrows its input. Its consumer trait `CanTryComputeRef` also supertraits `HasErrorType` and declares `fn try_compute_ref(&self, _code: PhantomData<Code>, input: &Input) -> Result<Self::Output, Self::Error>`, taking `&Input` where `CanTryCompute` takes `Input`. Both components are synchronous; their async-and-fallible counterpart is `Handler`.

## Implementations

A `TryComputer` provider is a zero-sized struct implementing the provider trait for a generic context that has an error type. Because the result names `Context::Error`, every fallible provider carries a `Context: HasErrorType` bound in its `where` clause. The crate's `ReturnInput` provider illustrates the minimal shape — it succeeds unconditionally, returning its input as the output:

```rust
impl<Context, Code, Input> TryComputer<Context, Code, Input> for ReturnInput
where
    Context: HasErrorType,
{
    type Output = Input;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(input)
    }
}
```

A `TryComputer` sits in the middle of the promotion lattice, so it both receives providers promoted from below and is promoted upward in turn. A plain [`Computer`](computer.md) becomes a `TryComputer` by wrapping its output in `Ok` — this is the `Promote` combinator — so an infallible provider satisfies `CanTryCompute` for free. In the other direction, a `Computer` whose output is *already* a `Result<Output, Context::Error>` becomes a `TryComputer` that unwraps that result, which is the `TryPromote` combinator; and a `TryComputer` is itself promoted to the async [`Handler`](handler.md) by wrapping it in a future. These promotions live in the [handler combinators](../providers/handler_combinators.md), so a provider author implements whichever single variant fits and lets the wiring bridge to `TryComputer`.

## Examples

A `TryComputer` provider that parses a string into a number, raising the context's error on failure, wires into a context like any other component:

```rust
use core::marker::PhantomData;
use cgp::prelude::*;
use cgp::extra::handler::{CanTryCompute, TryComputer, TryComputerComponent};

#[cgp_new_provider]
impl<Context, Code> TryComputer<Context, Code, String> for ParseU64
where
    Context: CanRaiseError<core::num::ParseIntError>,
{
    type Output = u64;

    fn try_compute(
        context: &Context,
        _code: PhantomData<Code>,
        input: String,
    ) -> Result<u64, Context::Error> {
        input.parse().map_err(|e| Context::raise_error(e))
    }
}

delegate_components! {
    App {
        TryComputerComponent: ParseU64,
    }
}
```

The provider `ParseU64` returns `Result<u64, Context::Error>`, converting the concrete `ParseIntError` into the context's abstract error with `CanRaiseError`. Because the consumer trait supertraits `HasErrorType`, the context must have an error type wired before it can call `try_compute` — `App` would delegate its `ErrorTypeProviderComponent` (and an error raiser) as well as `TryComputerComponent`. In everyday use the [`#[cgp_computer]`](../macros/cgp_computer.md) macro generates this kind of provider from a function returning `Result<u64, String>`, and wires the promotion table so the same function also answers `CanCompute` (returning the `Result` as its output), `CanHandle`, and the `Ref` variants.

## Related constructs

`TryComputer` is the fallible synchronous corner of the [handler family](../../concepts/handlers.md); its infallible counterpart is [`Computer`](computer.md), and its async-and-fallible generalization is [`Handler`](handler.md). It supertraits [`HasErrorType`](has_error_type.md), which supplies the `Self::Error` it returns, and a fallible provider typically uses [`CanRaiseError`](can_raise_error.md) to convert a concrete source error into that abstract error. The combinators that promote a `Computer` into a `TryComputer` (`Promote`, `TryPromote`) and a `TryComputer` into a `Handler` are documented in [handler combinators](../providers/handler_combinators.md). The macro that builds a `TryComputer` provider from a fallible function is [`#[cgp_computer]`](../macros/cgp_computer.md), and dispatching on `Code` or `Input` uses [`UseDelegate`](../providers/use_delegate.md) and the family's `UseInputDelegate`, per [dispatching](../../concepts/dispatching.md).

## Source

`TryComputer` and `TryComputerRef` are defined in [crates/extra/cgp-handler/src/components/try_compute.rs](../../../crates/extra/cgp-handler/src/components/try_compute.rs). The `ReturnInput` provider is in [crates/extra/cgp-handler/src/providers/return_input.rs](../../../crates/extra/cgp-handler/src/providers/return_input.rs), and the promotion combinators in [crates/extra/cgp-handler/src/providers/](../../../crates/extra/cgp-handler/src/). The components are re-exported through `cgp::extra::handler`. Behavioral tests exercising the fallible computers and their promotions are in [crates/tests/cgp-tests/tests/handler_tests/computer_macro.rs](../../../crates/tests/cgp-tests/tests/handler_tests/computer_macro.rs).
