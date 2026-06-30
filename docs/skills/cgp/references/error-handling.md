# Error handling

CGP's modular error handling: one shared abstract `Error` type per context, plus pluggable behavior for constructing that error from a source error and for attaching detail to it.

Generic CGP code must be able to fail without naming a concrete error type. A provider that hits a fallible operation gets back a `parse error`, an `io::Error`, or a string, yet it is generic over the context and cannot commit to `anyhow::Error` or any other choice — that choice belongs to the application assembling the context. CGP resolves this with three cooperating [components](components.md): `HasErrorType` gives the context one abstract `Self::Error`, `CanRaiseError<SourceError>` constructs that abstract error from a concrete source error, and `CanWrapError<Detail>` enriches an existing abstract error with extra detail. The concrete error type and the raising/wrapping behavior are decided once, at [wiring](wiring.md) time, by whichever error backend the context plugs in.

A note on imports: the consumer traits `HasErrorType`, `CanRaiseError`, and `CanWrapError` come through `use cgp::prelude::*;`, but the wiring keys and backend providers below do not. The component markers (`ErrorTypeProviderComponent`, `ErrorRaiserComponent`, `ErrorWrapperComponent`) live under `cgp::core::error`, and the backend providers (`RaiseFrom`, `ReturnError`, `DebugError`, `DisplayError`, and the rest) live under `cgp::extra::error`, so a module that wires error handling imports the specific names it uses, for example `use cgp::core::error::ErrorRaiserComponent;` and `use cgp::extra::error::RaiseFrom;`.

## `HasErrorType`: the shared abstract error

`HasErrorType` is the abstract-type component that gives a context a single shared `Error` type, and every fallible CGP operation refers to it. It is declared with `#[cgp_type]`, so it behaves like any other [abstract type](abstract-types.md) — a trait with one associated type, wired through a provider rather than hand-implemented:

```rust
#[cgp_type]
pub trait HasErrorType {
    type Error: Debug;
}

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
```

The `Debug` bound lets `Self::Error` flow into `.unwrap()` and straightforward logging without an extra constraint, and it is enforced on whatever concrete type a context chooses. `ErrorOf<Context>` is the convenient spelling of the associated-type path. Generic code that may fail returns `Result<T, Self::Error>` (or `Result<T, ErrorOf<Context>>`) and never names a concrete error.

Centralizing the error type on one trait is what lets errors compose. A context trait that may fail supertraits `HasErrorType`, so every such trait refers to the *same* `Self::Error`; if each declared its own associated `Error`, a context bounded by several of them would face several incompatible error types with no way to unify them. `HasErrorType` carries no methods — it only declares the type. The behavior of producing errors lives in the traits that supertrait it.

Because `#[cgp_type]` generates a `UseType` blanket impl, a context fixes its error type by wiring the error-type component to `UseType<E>`, exactly as for any abstract type:

```rust
#[cgp_component(Validator)]
pub trait CanValidate: HasErrorType {
    fn validate(&self) -> Result<(), Self::Error>;
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}
```

Here `CanValidate` supertraits `HasErrorType`, so its `Self::Error` is the context's shared abstract error, and `App` fixes that error to `String`. The standalone backends (`cgp-error-anyhow`, `cgp-error-eyre`, `cgp-error-std`) supply ready-made providers that set `Error` to their respective library types instead. A context can equally implement the trait directly — `impl HasErrorType for App { type Error = String; }` — which makes plain that it is an ordinary trait with a `Debug`-bounded associated type.

## `CanRaiseError` and `CanWrapError`: producing and enriching the error

`CanRaiseError<SourceError>` is the consumer trait for turning a concrete source error into the context's abstract `Self::Error`, and `CanWrapError<Detail>` is the companion that attaches detail to an existing one. Both supertrait `HasErrorType`, and both are `#[cgp_component]`s that delegate per type so a context can handle each source error or detail with a different provider:

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

`raise_error` takes the source error by value and returns the abstract error; `wrap_error` takes the current abstract error plus a `Detail` and returns an enriched one. Both are associated functions with no `self` receiver — raising and wrapping are properties of the context *type*, so generic code calls `Context::raise_error(source)` and `Context::wrap_error(err, detail)` where only the type parameter is in scope. The `#[cgp_component(...)]` attribute names the provider traits `ErrorRaiser` and `ErrorWrapper`, and `#[derive_delegate(UseDelegate<...>)]` makes each dispatch per `SourceError` or `Detail` type through a delegation table.

A provider written against these bounds names neither the context nor its concrete error type:

```rust
#[cgp_component(Loader)]
pub trait CanLoad: HasErrorType {
    fn load(&self, path: &str) -> Result<String, Self::Error>;
}

#[cgp_impl(new LoadOrFail)]
impl Loader for Context
where
    Context: CanRaiseError<String> + CanWrapError<String>,
{
    fn load(&self, path: &str) -> Result<String, Self::Error> {
        if path.is_empty() {
            let err = Context::raise_error("empty path".to_owned());
            return Err(Context::wrap_error(err, format!("while loading {path}")));
        }
        Ok(format!("contents of {path}"))
    }
}
```

The provider requires `CanRaiseError<String>` to turn a message into the abstract error and `CanWrapError<String>` to attach context as it propagates — both [impl-side dependencies](components.md) that any wired context satisfies by plugging in providers. The context decides, through wiring, what concrete error type `load` actually produces.

## Wiring the behavior: error-backend providers

A context gains raising and wrapping by wiring `ErrorRaiserComponent` and `ErrorWrapperComponent` to providers, exactly like any other component. The `cgp-error-extra` crate supplies a family of zero-sized providers (the [providers](components.md) are markers with no runtime value) that are generic over the context's error type and capture cross-cutting strategies independent of any one error library; they sit alongside the standalone backends, which specialize to a concrete library error such as `anyhow::Error`. A typical context wires a mix: a backend for the concrete error type plus these generic providers for the strategies. The raisers, wrappers, and their bounds are:

- `RaiseFrom` implements `ErrorRaiser` by converting through `From` — it raises any source error whose `Context::Error: From<E>`, the default choice when the abstract error already absorbs the source.
- `ReturnError` implements `ErrorRaiser` for the case where the source *is* the abstract error (`HasErrorType<Error = E>`), returning it untouched.
- `RaiseInfallible` implements `ErrorRaiser` for `core::convert::Infallible`, producing the error by an empty `match` that can never run — letting code parameterized over a fallible operation be wired uniformly even when the chosen operation cannot fail.
- `PanicOnError` implements `ErrorRaiser` by `panic!`-ing with the source error's `Debug` output instead of returning a value — for contexts that treat an error as a fail-fast fault, such as tests.
- `DiscardDetail` implements `ErrorWrapper` by dropping the detail and returning the error unchanged — the wrapping no-op, for error types that cannot carry extra context.
- `DebugError` and `DisplayError` implement *both* components by formatting the source error or detail into a `String` and forwarding to the context's own `CanRaiseError<String>` / `CanWrapError<String>` — `DebugError` via `Debug`, `DisplayError` via `Display`/`to_string()`. Both live behind the crate's `alloc` feature.

The simplest wiring delegates to a single provider. Wiring `RaiseFrom` lets `App` raise any source error its abstract error implements `From` for:

```rust
delegate_components! {
    App {
        ErrorRaiserComponent: RaiseFrom,
    }
}
```

The string-formatting providers are designed to *compose* with the others rather than replace them: `DebugError` and `DisplayError` do not know the context's error type, they only reduce a `Debug` or `Display` value to a `String` and hand it off, so the context must separately wire a provider that handles the `String` source. Because both components dispatch per source-error type, the idiomatic wiring lists one entry per source error — a concrete string-raising rule plus formatting redirects for everything else — opened on the component with the `open` statement of `delegate_components!`:

```rust
delegate_components! {
    App {
        open { ErrorRaiserComponent };

        @ErrorRaiserComponent.String: RaiseFrom,
        @ErrorRaiserComponent.ParseError: DebugError,
    }
}
```

The `open { ErrorRaiserComponent };` header opens the component for per-type wiring, and each `@ErrorRaiserComponent.<SourceError>: Provider` entry assigns the provider for one source-error type, folded directly into `App`'s own table — `open` needs no `#[derive_delegate]` of its own because every `#[cgp_component]` already generates the `RedirectLookup` impl it dispatches through. The legacy equivalent writes the same per-type entries into a separate `UseDelegate<new AppErrorRaisers { String: RaiseFrom, ParseError: DebugError }>` nested table; that form is still common in existing code but is slated for deprecation, so prefer `open` for new wiring. See [wiring](wiring.md) for both forms.

A raised `String` is converted straight into the abstract error by `RaiseFrom`, while a raised `ParseError` is formatted with `Debug` by `DebugError` and then routed back through the `String` entry — which `RaiseFrom` handles — yielding one coherent error type from two unrelated sources. The choice of provider is therefore also a statement about which source errors the context accepts and how, and each wiring is verified with `check_components!` like any other.

## Related constructs

`HasErrorType` is an [abstract type](abstract-types.md) declared with `#[cgp_type]`, so it is wired with `UseType<E>` or a backend provider the same way every abstract type is. `CanRaiseError` and `CanWrapError` are ordinary [components](components.md) that supertrait it, wired with `delegate_components!` as covered in [wiring](wiring.md), and their `#[derive_delegate(UseDelegate<...>)]` is what lets a context dispatch raising and wrapping per source-error or detail type through a delegation table.

Further reference (online): [components/has_error_type.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/has_error_type.md), [components/can_raise_error.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/can_raise_error.md), [providers/error_providers.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/error_providers.md).
