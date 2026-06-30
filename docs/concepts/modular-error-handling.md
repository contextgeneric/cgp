# Modular error handling

CGP handles errors modularly by splitting error handling into three independent decisions — what the abstract error type is, how a foreign error is turned into it, and how detail is attached to it — each made by wiring rather than baked into the code that fails.

## The problem with a fixed error type

Generic code must be able to fail without committing to a concrete error type. A provider deep in a call graph encounters a parse error, an I/O error, or a domain rule violation, and it has to return *some* error — but it is generic over the context and cannot decide whether the application wants `anyhow::Error`, `std::io::Error`, a boxed trait object, or a hand-rolled enum. Hard-coding any of these into the provider would tie every caller to that choice, and converting between them by hand at each boundary is the boilerplate that error-handling crates exist to remove.

CGP turns the choice of error type, and the choice of how errors are constructed, into wiring decisions that live in one place per application. The code that fails refers only to an abstract error and to the capabilities of raising and wrapping; a concrete context supplies the error type and the construction strategy when it is assembled. Swapping `anyhow` for `eyre`, or routing one source error through `From` while formatting another into a string, changes a few wiring lines and touches no provider. This is the same [coherence](coherence.md)-bypassing move CGP applies everywhere — keep the implementations open and decide locally per context — applied specifically to error handling.

## The abstract error type

The anchor is [`HasErrorType`](../reference/components/has_error_type.md), a one-line abstract-type component that gives a context a single shared `Error` type. Every fallible operation returns `Result<T, Self::Error>` (or the alias `ErrorOf<Context>`), so generic code names the error without knowing its concrete identity:

```rust
#[cgp_type]
pub trait HasErrorType {
    type Error: Debug;
}
```

Centralizing the error on one trait is what lets errors compose across components. If each fallible component declared its own associated `Error`, a context bounded by several of them would face several unrelated error types with no way to unify them; by having every fallible trait supertrait `HasErrorType`, they all refer to the *same* `Self::Error`. The `Debug` bound is the only requirement the abstract error carries, enough for `.unwrap()` and logging, and it propagates to whatever concrete type a context eventually wires in. A context fixes that type either directly — `impl HasErrorType for App { type Error = anyhow::Error; }` — or, more commonly, by wiring its error-type component to a provider such as `UseType<AppError>`, since `#[cgp_type]` makes `HasErrorType` an abstract-type component built on [`HasType`/`TypeProvider`](../reference/components/has_type.md).

## Raising and wrapping as capabilities

Two further components give the abstract error its behavior, each parameterized so a context can handle many error shapes. [`CanRaiseError<SourceError>`](../reference/components/can_raise_error.md) converts a concrete source error into the context's abstract error, and its companion `CanWrapError<Detail>` attaches a piece of detail to an existing one. Both supertrait `HasErrorType`, so the error they produce and enrich is the context's shared `Self::Error`:

```rust
#[cgp_component(ErrorRaiser)]
#[derive_delegate(UseDelegate<SourceError>)]
pub trait CanRaiseError<SourceError>: HasErrorType {
    fn raise_error(error: SourceError) -> Self::Error;
}

#[cgp_component(ErrorWrapper)]
#[derive_delegate(UseDelegate<Detail>)]
pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(error: Self::Error, detail: Detail) -> Self::Error;
}
```

These two capabilities cover the everyday error-handling motions: raise a foreign error into the abstract one, and wrap context onto it as it propagates. A provider that fails writes `Context::raise_error(source)` and `Context::wrap_error(err, detail)` — both associated functions, called on the context *type*, because constructing an error is a property of the context rather than of any value in scope. Crucially, the provider states which sources it raises and which details it wraps as [impl-side dependencies](impl-side-dependencies.md) in its `where` clause, so those requirements never leak into the consumer trait a caller bounds on. A loader, for example, needs only `Context: CanRaiseError<String> + CanWrapError<String>` to produce and enrich an error it knows nothing concrete about.

## Strategies as interchangeable providers

Because raising and wrapping are components, the *strategy* for each becomes a provider a context selects, and CGP ships a family of them in `cgp-error-extra` that stay generic over the context's error type. Each captures one cross-cutting way to handle an error, independent of any particular error library, so an application composes its error handling from these parts rather than writing conversion glue. The most common are a small set worth knowing by name:

- `RaiseFrom` raises a source error by converting it through the standard `From` trait, the default whenever the abstract error already absorbs the source.
- `ReturnError` is the identity raiser for when the source already *is* the abstract error.
- `RaiseInfallible` absorbs `core::convert::Infallible`, letting code generic over a fallible step wire uniformly even when that step cannot fail.
- `DebugError` and `DisplayError` format any `Debug` or `Display` source into a `String` and forward to the context's *own* `CanRaiseError<String>`, reducing an open-ended set of error types to the one string case the context handles.
- `DiscardDetail` wraps by dropping the detail, and `PanicOnError` aborts instead of producing an error value.

The full list and the exact bound each provider places on the context live in the [error providers reference](../reference/providers/error_providers.md). What matters at the concept level is that these are plain providers wired like any other, so the choice of provider is also a statement about which source errors a context accepts and how it treats them.

## Pluggable concrete backends

Sitting alongside the generic strategy providers are the standalone backends, each pinning the abstract error to one concrete library type and supplying the raisers and wrappers that go with it. The `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std` crates each export a type-setting provider — `UseAnyhowError` sets `Self::Error` to `anyhow::Error`, for instance — together with providers like `RaiseAnyhowError` and `DebugAnyhowError` that raise and wrap into that concrete type. Wiring a backend is what makes the abstract error concrete:

```rust
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseAnyhowError,
        ErrorRaiserComponent: RaiseAnyhowError,
    }
}
```

The backend is opt-in and confined to these wiring lines, which is the whole point of keeping it separate. Choosing `eyre` instead is a matter of swapping `UseAnyhowError`/`RaiseAnyhowError` for their `eyre` counterparts, and no provider that calls `raise_error` changes, because none of them named the concrete type in the first place. A typical application wires a mix: a backend for the concrete error type, plus the generic strategy providers for the cross-cutting cases the backend does not cover.

## Dispatching per source error type

The reason `CanRaiseError` and `CanWrapError` are parameterized rather than monomorphic is that a single context usually raises several unrelated source errors, each best handled differently — and the per-type dispatch each component derives is what lets one component fan out to a provider per source. Because both carry `#[derive_delegate(UseDelegate<SourceError>)]`, a context can open the component and assign a provider to each source error type directly in its own table, the same per-type wiring used for [dispatching](dispatching.md) any generic parameter:

```rust
delegate_components! {
    App {
        open {ErrorRaiserComponent};

        @ErrorRaiserComponent.String: RaiseFrom,
        @ErrorRaiserComponent.ParseIntError: DebugError,
    }
}
```

Here a raised `String` is converted straight into the abstract error by `RaiseFrom`, while a `ParseIntError` is formatted with `Debug` and then forwarded back through the `String` entry — which `RaiseFrom` handles — so two different sources collapse into one coherent error type. The `open` statement and its `@`-path keys are the [namespace](namespaces.md)-based wiring form; the older [`UseDelegate`](../reference/providers/use_delegate.md) provider expresses the same dispatch through a separate nested table. Either way, the context decides the routing, and the formatting providers like `DebugError` compose with the converting ones rather than replacing them.

## Application-specific error components

Nothing about modular error handling is confined to the built-in components; an application defines its own error-raising components the same way when its errors carry domain structure. A web service, for instance, wants every raised error to carry an HTTP status code, so it declares a component that takes both a status marker and a detail, and dispatches on the pair:

```rust
#[cgp_component(HttpErrorRaiser)]
pub trait CanRaiseHttpError<Code, Detail>: HasErrorType {
    fn raise_http_error(_code: Code, detail: Detail) -> Self::Error;
}
```

The markers `ErrUnauthorized`, `ErrNotFound`, and the like are empty structs that map to status codes, and a provider such as `DisplayHttpError` builds the concrete application error from the code and a `Display` detail. The context then wires this custom component exactly as it wires the built-in ones — opening it and routing each detail type to a provider — so a handler deep in the request pipeline writes `Self::raise_http_error(ErrUnauthorized, "you must first login".into())` without knowing the concrete error type or how status codes are attached. This is the modular error-handling pattern carried up to the application's own vocabulary: the same split between an abstract error, a raising capability, and a wiring choice, applied to errors that are richer than a plain message.

## Related constructs

Modular error handling is built entirely from ordinary CGP constructs. The abstract error is [`HasErrorType`](../reference/components/has_error_type.md), an [abstract-type](abstract-types.md) component defined with [`#[cgp_type]`](../reference/macros/cgp_type.md); the raising and wrapping capabilities are [`CanRaiseError` / `CanWrapError`](../reference/components/can_raise_error.md), and the strategies that satisfy them are the [error providers](../reference/providers/error_providers.md) plus the standalone backends. A provider declares which errors it raises through [impl-side dependencies](impl-side-dependencies.md) in its `where` clause, and a context selects a strategy per source error type through the [dispatching](dispatching.md) machinery and the [`open` statement](../reference/macros/delegate_components.md) over [namespaces](namespaces.md). The whole approach is the [coherence](coherence.md)-bypassing strategy specialized to errors: keep the raising implementations overlapping and open, and let each context restore a single coherent error type by wiring. The [modular serialization](../examples/modular-serialization.md) example wires an `anyhow` backend to make its context-supplied deserializer fallible, and the [money-transfer API](../examples/money-transfer-api.md) example raises status-coded HTTP errors through an application-specific error component of exactly this shape.

## Source

The abstract error type and the raising and wrapping traits are defined in [crates/core/cgp-error/src/](../../crates/core/cgp-error/src/); the generic strategy providers are in [crates/extra/cgp-error-extra/src/](../../crates/extra/cgp-error-extra/src/), and the pluggable concrete backends in [crates/standalone/error/](../../crates/standalone/error/).
