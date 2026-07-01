# `HasErrorType`

`HasErrorType` is the abstract-type component that gives a context a single, shared abstract `Error` type, decoupling CGP code from any concrete error implementation, with `ErrorOf<Context>` as the alias for the resolved type.

## Purpose

`HasErrorType` exists so that generic CGP code can fail without naming a concrete error type. A provider that may error needs to produce *some* error, but it is generic over the context and cannot commit to `anyhow::Error`, `std::io::Error`, or any other concrete choice; that choice belongs to the application assembling the context. `HasErrorType` resolves this by giving the context one abstract `Self::Error` type that every fallible operation refers to. Generic code returns `Result<T, Self::Error>` (or `ErrorOf<Context>`), and the concrete error type is decided once, at wiring time, by whichever error backend the context plugs in.

Centralizing the error type on one trait also avoids ambiguity. If each context trait declared its own associated `Error`, a context bounded by several of them would face multiple distinct `Self::Error` types with no way to unify them. By having context traits supertrait `HasErrorType`, every fallible component refers to the *same* `Self::Error`, so errors compose across components. This single shared abstract error is the anchor that [`CanRaiseError`](can_raise_error.md) and [`CanWrapError`](can_raise_error.md) build on.

## Definition

`HasErrorType` is an abstract-type component: a trait with one associated type, defined with [`#[cgp_type]`](../macros/cgp_type.md). Its source is:

```rust
#[cgp_type]
#[prefix(@cgp.core.error in DefaultNamespace)]
pub trait HasErrorType {
    type Error: Debug;
}

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
```

The associated `Error` type is the context's abstract error, and its `Debug` bound is required so that `Self::Error` can be used in `.unwrap()` calls and in straightforward error logging without a separate constraint. The `ErrorOf<Context>` alias is the convenient spelling of `<Context as HasErrorType>::Error`, used wherever writing the full associated-type path would be noise.

Because the trait is declared with `#[cgp_type]`, it is a full abstract-type component rather than a plain trait: the macro generates a provider trait (`ErrorTypeProvider`), the component marker, the consumer and provider blanket impls, and a [`UseType`](../providers/use_type.md) blanket impl. The `Debug` bound on `Error` is carried through every generated construct, so any concrete error wired in must implement `Debug`. The `#[prefix(...)]` attribute places the generated names in the error namespace.

## Behavior

A context obtains its abstract error type either by implementing `HasErrorType` directly — `impl HasErrorType for App { type Error = anyhow::Error; }` — or, more commonly, by wiring its error-type component to a provider. Because `#[cgp_type]` generates the `UseType` impl, a context can name the concrete error type directly in its wiring with `UseType<E>`, and the standalone error backends (the `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std` crates) provide ready-made providers that set `Error` to their respective error types. Once the context has `HasErrorType`, every component bounded by it shares that one `Self::Error`.

`HasErrorType` carries no methods of its own — it only declares the type. The behavior of producing errors lives in the traits that supertrait it: [`CanRaiseError`](can_raise_error.md) converts a source error into `Self::Error`, and [`CanWrapError`](can_raise_error.md) adds detail to an existing `Self::Error`. This separation keeps the type declaration independent of any particular way of constructing the error.

## Examples

A context declares its abstract error and generic code returns it without naming a concrete type:

```rust
use cgp::prelude::*;

#[cgp_component(Validator)]
#[use_type(HasErrorType::Error)]
pub trait CanValidate {
    fn validate(&self) -> Result<(), Error>;
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}
```

Here [`#[use_type(HasErrorType::Error)]`](../attributes/use_type.md) adds `HasErrorType` as a supertrait of `CanValidate` and rewrites the bare `Error` to `<Self as HasErrorType>::Error`, so `validate` returns the context's shared abstract error without writing `Self::Error`. `App` wires its error-type component to `UseType<String>`, fixing that error to `String` (which satisfies the `Debug` bound). The same abstract error can equally be implemented directly:

```rust
impl HasErrorType for App {
    type Error = String;
}
```

This direct form makes plain that `HasErrorType` is an ordinary trait with a `Debug`-bounded associated type.

## Related constructs

`HasErrorType` is defined with [`#[cgp_type]`](../macros/cgp_type.md), which makes it an abstract-type component and generates its `UseType` provider; it is therefore a concrete instance of the [`HasType`/`TypeProvider`](has_type.md) machinery that `#[cgp_type]` builds every abstract type on. The traits that give the abstract error its behavior are [`CanRaiseError`](can_raise_error.md), which raises a source error into `Self::Error`, and [`CanWrapError`](can_raise_error.md), which wraps additional detail onto it — both supertrait `HasErrorType`. The [modular error handling](../../concepts/modular-error-handling.md) concept ties this trait together with those capabilities and the providers that satisfy them into the wider error-handling strategy.

## Source

The trait and the `ErrorOf` alias are defined in [crates/core/cgp-error/src/traits/has_error_type.rs](../../../crates/core/cgp-error/src/traits/has_error_type.rs). The `#[cgp_type]` machinery it relies on lives in [crates/macros/cgp-macro-core/src/types/cgp_type/](../../../crates/macros/cgp-macro-core/src/types/cgp_type/), and the underlying `HasType`/`TypeProvider`/`UseType` definitions are in [crates/core/cgp-type/src/](../../../crates/core/cgp-type/src/). The pluggable concrete error backends are in [crates/standalone/error/](../../../crates/standalone/error/). Behavioral tests are in [crates/tests/cgp-tests](../../../crates/tests/cgp-tests).
