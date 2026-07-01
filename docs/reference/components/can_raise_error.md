# `CanRaiseError`

`CanRaiseError<SourceError>` is the consumer trait for turning a concrete source error into a context's abstract `Self::Error`, with the companion `CanWrapError<Detail>` adding detail to an existing abstract error; both build on [`HasErrorType`](has_error_type.md).

## Purpose

`CanRaiseError` exists so that generic CGP code can produce its context's abstract error from any concrete error it encounters. A provider that calls a fallible operation gets back a specific error type — a parse error, an I/O error, a string message — but it must return the context's abstract `Self::Error`, whose concrete identity it does not know. `CanRaiseError<SourceError>` bridges the gap: it converts a value of the concrete `SourceError` into `Self::Error`, so generic code can write `Context::raise_error(source)` and let the context decide how that source maps into its chosen error type. Because the trait is parameterized by `SourceError`, a single context can know how to raise many different source errors into the one abstract error.

`CanWrapError` solves the complementary problem of enriching an error as it propagates. Rather than converting a foreign error in, it takes an error the context already holds and attaches a piece of `Detail` to it — a context message, a span, a path — producing an enriched `Self::Error`. The two together cover the common error-handling motions in CGP: raise a foreign error into the abstract one, and wrap context onto it as it bubbles up.

## Definition

Both traits import the context's shared abstract error type with [`#[use_type(HasErrorType::Error)]`](../attributes/use_type.md), so a bare `Error` in either signature stands for the context's error rather than being written as `Self::Error`. Each is a `#[cgp_component]`, making it a full component with a generated provider trait. `CanRaiseError` converts a source error into the abstract error:

```rust
#[cgp_component(ErrorRaiser)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<SourceError>)]
#[use_type(HasErrorType::Error)]
pub trait CanRaiseError<SourceError> {
    fn raise_error(error: SourceError) -> Error;
}
```

The `SourceError` parameter is the concrete error being raised, and `raise_error` is an associated function — it takes the source error by value and returns the abstract `Error`, without needing a `self` receiver, because raising an error is a property of the context type rather than of any particular value. `#[use_type(HasErrorType::Error)]` adds `HasErrorType` as a supertrait and rewrites the bare `Error` to `<Self as HasErrorType>::Error`. The `#[cgp_component(ErrorRaiser)]` attribute names the provider trait `ErrorRaiser`, and [`#[derive_delegate(UseDelegate<SourceError>)]`](../attributes/derive_delegate.md) wires `UseDelegate` so the raise behavior can be dispatched per source-error type through a delegation table — a context can handle each `SourceError` with a different provider.

`CanWrapError` has the same shape but takes an existing error plus a detail:

```rust
#[cgp_component(ErrorWrapper)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<Detail>)]
#[use_type(HasErrorType::Error)]
pub trait CanWrapError<Detail> {
    fn wrap_error(error: Error, detail: Detail) -> Error;
}
```

Here `wrap_error` takes the context's current `Error` and a `Detail` value and returns a new `Error` with the detail folded in. Its provider trait is `ErrorWrapper`, and it delegates per `Detail` type, so wrapping a string message and wrapping a structured detail can be handled by different providers.

## Behavior

A context gains these capabilities by wiring `ErrorRaiserComponent` and `ErrorWrapperComponent` to providers, exactly as for any other component. Because both traits delegate through `UseDelegate<SourceError>` and `UseDelegate<Detail>`, the natural wiring is a delegation table that maps each concrete source-error or detail type to a provider that knows how to handle it; a context can therefore raise a handful of unrelated error types into one abstract error, each through its own provider. The pluggable error backends (`cgp-error-anyhow`, `cgp-error-eyre`, `cgp-error-std`) supply providers that implement these traits for common cases, so an application usually wires a backend rather than writing the raise and wrap logic itself.

Both traits being associated-function components means `raise_error` and `wrap_error` are called on the context *type* — `Context::raise_error(source)` — and produce the abstract error without borrowing the context value. This matches how errors are typically constructed deep inside generic code where only the type parameter is in scope.

## Examples

A provider raises a concrete error into the abstract one and wraps a message onto it as it propagates:

```rust
use cgp::prelude::*;

#[cgp_component(Loader)]
#[use_type(HasErrorType::Error)]
pub trait CanLoad {
    fn load(&self, path: &str) -> Result<String, Error>;
}

#[cgp_impl(new LoadOrFail)]
#[uses(CanRaiseError<String>, CanWrapError<String>)]
#[use_type(HasErrorType::Error)]
impl Loader {
    fn load(&self, path: &str) -> Result<String, Error> {
        if path.is_empty() {
            let err = Self::raise_error("empty path".to_owned());
            return Err(Self::wrap_error(err, format!("while loading {path}")));
        }
        Ok(format!("contents of {path}"))
    }
}
```

The provider names neither the context nor its concrete error type. It requires `CanRaiseError<String>` to turn a `String` message into the abstract error and `CanWrapError<String>` to attach further context, and any wired context that satisfies those bounds — typically by plugging in an error backend — makes `load` produce errors in that context's chosen error type.

## Related constructs

`CanRaiseError` and `CanWrapError` both supertrait [`HasErrorType`](has_error_type.md), whose abstract `Self::Error` they produce and enrich. Their delegation is configured by [`#[derive_delegate(UseDelegate<...>)]`](../attributes/derive_delegate.md), so a context dispatches per source-error or detail type through a delegation table. Both are ordinary `#[cgp_component]` components, wired with `delegate_components!` and checked with `check_components!`. The [modular error handling](../../concepts/modular-error-handling.md) concept frames how these capabilities, the abstract error type, and the strategy providers combine into wiring-time error-handling decisions.

## Source

- `CanRaiseError` is defined in [crates/core/cgp-error/src/traits/can_raise_error.rs](../../../crates/core/cgp-error/src/traits/can_raise_error.rs) and `CanWrapError` in [crates/core/cgp-error/src/traits/can_wrap_error.rs](../../../crates/core/cgp-error/src/traits/can_wrap_error.rs).
- Both build on `HasErrorType` from [crates/core/cgp-error/src/traits/has_error_type.rs](../../../crates/core/cgp-error/src/traits/has_error_type.rs).
- The pluggable providers that implement them live in [crates/standalone/error/](../../../crates/standalone/error/).
