# In-tree error providers

The in-tree error providers are the zero-sized provider structs in `cgp-error-extra` that implement the error-raising and error-wrapping components for any context, independent of which concrete error type that context has chosen.

## Purpose

These providers exist so that a context can wire up common error-handling strategies without depending on a specific error library. The [`CanRaiseError`](../components/can_raise_error.md) and `CanWrapError` components define *what* a context can do with errors — turn a foreign error into its abstract `Self::Error`, or attach detail to an existing one — but they say nothing about *how*. The providers here supply the how for the cases that do not need a particular backend: convert through `From`, return an error that already is the abstract type, format a foreign error into a string, discard a detail, panic, or absorb an impossible error. Each is generic over the context, so it works with whatever error type the context's [`HasErrorType`](../components/has_error_type.md) names.

These are the in-tree counterparts to the standalone error backends in `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std`. Those crates supply providers specialized to a concrete error type — raising into an `anyhow::Error`, for example — whereas the providers in `cgp-error-extra` stay abstract over the context's error type and capture strategies that are independent of any one library. A context typically wires a mix: a backend for the concrete error type plus these generic providers for the cross-cutting strategies.

Each provider implements one or both of the two error components through their provider traits. `CanRaiseError`'s provider trait is `ErrorRaiser`, wired to the context with `ErrorRaiserComponent`; `CanWrapError`'s provider trait is `ErrorWrapper`, wired with `ErrorWrapperComponent`. A provider that implements `ErrorRaiser` supplies raising; one that implements `ErrorWrapper` supplies wrapping; some supply both.

## Implementations

The six providers divide into three groups by which component they implement and how they treat the error. The pure raisers — `RaiseFrom`, `ReturnError`, and `RaiseInfallible` — implement `ErrorRaiser` to convert a source error into the abstract error. `DiscardDetail` implements `ErrorWrapper` to drop detail. `PanicOnError` implements `ErrorRaiser` to abort rather than produce an error value. The string-formatting providers `DebugError` and `DisplayError` implement both components, redirecting through a string. The following sections describe each, naming the component it provides and the bound it places on the context or the error.

### `RaiseFrom` — convert via `From`

`RaiseFrom` is the `ErrorRaiser` provider that raises a source error by converting it into the context's error type with the standard `From` trait. It implements `ErrorRaiser<Context, E>` for any context whose abstract `Error` implements `From<E>`:

```rust
#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for RaiseFrom
where
    Context: HasErrorType,
    Context::Error: From<E>,
{
    fn raise_error(e: E) -> Context::Error {
        e.into()
    }
}
```

This is the default choice whenever the abstract error already knows how to absorb the source error through `From`. Because the bound is `Context::Error: From<E>`, a single wiring of `RaiseFrom` covers every source error type the context's error has a `From` impl for.

### `ReturnError` — the source is already the abstract error

`ReturnError` is the `ErrorRaiser` provider for the case where the source error *is* the context's abstract error, so raising is the identity. It implements `ErrorRaiser<Context, E>` only when the context's `Error` is exactly `E`:

```rust
#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for ReturnError
where
    Context: HasErrorType<Error = E>,
{
    fn raise_error(e: E) -> E {
        e
    }
}
```

The `HasErrorType<Error = E>` bound ties the source type to the abstract error, so `raise_error` returns its argument untouched. This is what a context uses when generic code raises a value that is already of the context's chosen error type.

### `RaiseInfallible` — absorb an impossible error

`RaiseInfallible` is the `ErrorRaiser` provider for `core::convert::Infallible`, the error type that can never be constructed. It implements `ErrorRaiser<Context, Infallible>` for any context with an error type, producing the abstract error by matching on the uninhabited value:

```rust
#[cgp_new_provider]
impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
```

Since an `Infallible` value cannot exist, the empty `match` is total and the function is never actually called at runtime. This provider lets generic code that is parameterized over a fallible operation be wired uniformly even when the operation chosen for a given context cannot fail.

### `DiscardDetail` — wrap by ignoring the detail

`DiscardDetail` is the `ErrorWrapper` provider that throws away whatever detail is attached and returns the error unchanged. It implements `ErrorWrapper<Context, Detail>` for any context and any detail type:

```rust
#[cgp_new_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DiscardDetail
where
    Context: HasErrorType,
{
    fn wrap_error(error: Context::Error, _detail: Detail) -> Context::Error {
        error
    }
}
```

This satisfies the `CanWrapError` capability without actually enriching the error, which is useful when a context's error type cannot carry extra context, or when the wrapping detail is deliberately not retained. It is the wrapping counterpart to a no-op: the error propagates as-is.

### `PanicOnError` — abort instead of producing an error

`PanicOnError` is the `ErrorRaiser` provider that panics with the source error's debug representation rather than returning an abstract error. It implements `ErrorRaiser<Context, E>` for any context whose error type exists, requiring only that the source error is `Debug`:

```rust
#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for PanicOnError
where
    Context: HasErrorType,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        panic!("{e:?}")
    }
}
```

Although the signature promises to return `Context::Error`, the body never does — `panic!` diverges. This provider is for contexts where an error is treated as a programming fault that should abort rather than be handled, such as tests or fail-fast tooling.

### `DebugError` and `DisplayError` — format through a string

`DebugError` and `DisplayError` implement *both* error components by redirecting through the context's string-based error handling. Rather than producing the abstract error directly, each formats the source error or detail into a `String` and forwards to the context's own `CanRaiseError<String>` or `CanWrapError<String>` — so they delegate the final step to whatever string-handling provider the context already wires. `DebugError` formats with the `Debug` trait:

```rust
#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for DebugError
where
    Context: CanRaiseError<String>,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(format!("{e:?}"))
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugError
where
    Context: CanWrapError<String>,
    Detail: Debug,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Context::wrap_error(error, format!("{detail:?}"))
    }
}
```

`DisplayError` is identical in shape but formats with the `Display` trait and `to_string()` instead, raising `Context::raise_error(e.to_string())` and wrapping `Context::wrap_error(error, detail.to_string())`. Both require the context to already raise and wrap `String`, which is the indirection that lets them reduce any `Debug` or `Display` error to the string case the context knows how to handle. Because they format into an allocated `String`, both live behind the crate's `alloc` feature.

## Behavior

A context gains an error strategy by wiring one of these providers to `ErrorRaiserComponent` or `ErrorWrapperComponent` exactly like any other component, and the provider's `where` clause is what determines when that wiring type-checks. `RaiseFrom` requires the abstract error to be `From` the source, `ReturnError` requires the source to be the abstract error itself, `RaiseInfallible` accepts only `Infallible`, and `PanicOnError` accepts any `Debug` source — so the choice of provider is also a statement about which source errors a context will accept and how. Because both components dispatch through `UseDelegate` over the source-error or detail type, a context commonly wires several of these providers at once through a delegation table, one per source error type.

The string-formatting providers compose with the other raisers rather than replacing them, which is the key to their design. `DebugError` and `DisplayError` do not know the context's error type; they only know how to turn a `Debug` or `Display` value into a `String` and hand it off. The context must separately wire a provider — often `RaiseFrom` or a backend provider — that handles the `String` source, and the formatting providers route every other source error through that single string path. This lets a context handle an open-ended set of error types with one concrete string-raising rule plus a uniform formatting redirect.

## Examples

Wiring `RaiseFrom` to the error-raiser component lets a context raise any source error its abstract error can absorb through `From`:

```rust
use cgp::prelude::*;
use cgp::core::error::ErrorRaiserComponent;
use cgp::extra::error::RaiseFrom;

delegate_components! {
    App {
        ErrorRaiserComponent:
            RaiseFrom,
    }
}
```

With this wiring, any generic provider that calls `Context::raise_error(source)` on `App` succeeds for every `source` whose type the `App` error implements `From` for.

A context can combine the formatting and converting providers by dispatching per source-error type, so that a `String` is raised directly while other `Debug` errors are formatted into a string first:

```rust
use cgp::prelude::*;
use cgp::core::error::ErrorRaiserComponent;
use cgp::extra::error::{DebugError, RaiseFrom};

delegate_components! {
    App {
        ErrorRaiserComponent:
            UseDelegate<new AppErrorRaisers {
                String:
                    RaiseFrom,
                ParseError:
                    DebugError,
            }>,
    }
}
```

Here a raised `String` is converted straight into the abstract error by `RaiseFrom`, while a raised `ParseError` is formatted with `Debug` by `DebugError` and then forwarded back through the `String` entry — which `RaiseFrom` handles — giving a single coherent error type from two different sources.

## Related constructs

These providers implement the [`CanRaiseError`](../components/can_raise_error.md) and `CanWrapError` components through their `ErrorRaiser` and `ErrorWrapper` provider traits, and every one of them is generic over a context that supplies [`HasErrorType`](../components/has_error_type.md) to name the abstract `Self::Error` they produce. They are wired to a context with `delegate_components!` on `ErrorRaiserComponent`/`ErrorWrapperComponent` and checked with `check_components!`, and because both components derive `UseDelegate`, they are typically dispatched per source-error or detail type through a delegation table — see [`UseDelegate`](use_delegate.md). The library-specific counterparts that raise into a concrete error type live in the standalone backends `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std`. The [modular error handling](../../concepts/modular-error-handling.md) concept explains how these providers fit alongside the abstract error type and the backends as interchangeable error-handling strategies.

## Source

- The providers are defined in `cgp-error-extra`: `RaiseFrom` in [crates/extra/cgp-error-extra/src/impls/raise_from.rs](../../../crates/extra/cgp-error-extra/src/impls/raise_from.rs), `ReturnError` in [return_error.rs](../../../crates/extra/cgp-error-extra/src/impls/return_error.rs), `RaiseInfallible` in [infallible.rs](../../../crates/extra/cgp-error-extra/src/impls/infallible.rs), `DiscardDetail` in [discard_detail.rs](../../../crates/extra/cgp-error-extra/src/impls/discard_detail.rs), `PanicOnError` in [panic_error.rs](../../../crates/extra/cgp-error-extra/src/impls/panic_error.rs), and `DebugError`/`DisplayError` (behind the `alloc` feature) in [impls/alloc/debug_error.rs](../../../crates/extra/cgp-error-extra/src/impls/alloc/debug_error.rs) and [display_error.rs](../../../crates/extra/cgp-error-extra/src/impls/alloc/display_error.rs).
- The `ErrorRaiser`/`ErrorWrapper` provider traits and the `CanRaiseError`/`CanWrapError`/`HasErrorType` consumer traits they build on are in [crates/core/cgp-error/src/](../../../crates/core/cgp-error/src/).
- The standalone backend counterparts are in [crates/standalone/error/](../../../crates/standalone/error/).
