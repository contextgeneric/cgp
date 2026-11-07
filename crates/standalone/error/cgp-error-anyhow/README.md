[![Crates.io](https://img.shields.io/crates/v/cgp-error-anyhow.svg)](https://crates.io/crates/cgp-error-anyhow)
[![Docs.rs](https://docs.rs/cgp-error-anyhow/badge.svg)](https://docs.rs/cgp-error-anyhow)
![License](https://img.shields.io/badge/license-Apache_2.0-blue.svg)

# cgp-error-anyhow: anyhow as a context's error type

`cgp-error-anyhow` makes [`anyhow::Error`](https://docs.rs/anyhow) the abstract error type of a
[CGP](https://contextgeneric.dev) context, and supplies the providers that raise errors into it and
add context to it. Code written against `HasErrorType`, `CanRaiseError`, and `CanWrapError` stays
generic; only the context's wiring names anyhow.

**[Website](https://contextgeneric.dev/) · [Modular error handling](https://contextgeneric.dev/docs/concepts/modular-error-handling) · [The error components](https://contextgeneric.dev/docs/reference/components/has_error_type) · [Project record](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/projects/error/README.md)**

| Provider | Wire it to | What it does |
|---|---|---|
| `UseAnyhowError` | `ErrorTypeProviderComponent` | sets the context's `Error` to `anyhow::Error` |
| `RaiseAnyhowError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises a standard error without formatting it, so `downcast_ref` still finds it; wraps a `Display + Send + Sync + 'static` detail as anyhow context |
| `DebugAnyhowError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises or wraps any `Debug` value as a message formatted with `{:?}` |
| `DisplayAnyhowError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises or wraps any `Display` value as a message formatted with `{}` |

A context usually routes each source type to the provider that suits it:

```rust,ignore
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{DisplayAnyhowError, RaiseAnyhowError, UseAnyhowError};

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseAnyhowError,
        @ErrorRaiserComponent.std::io::Error: RaiseAnyhowError,
        @ErrorRaiserComponent.String: DisplayAnyhowError,
        ErrorWrapperComponent: RaiseAnyhowError,
    }
}

let error = App::raise_error(std::io::Error::other("disk full"));
let error = App::wrap_error(error, "while saving");
assert_eq!(format!("{error:#}"), "while saving: disk full");
```

A `String` needs `DisplayAnyhowError` or `DebugAnyhowError`, because it is not a standard error.
`DebugAnyhowError` prints it with quotes. The crate is `no_std` and builds anyhow without its `std`
feature. Enabling that feature elsewhere in the dependency graph turns on the parts of anyhow that
need `std`, such as backtrace capture.

The crate re-exports `anyhow::Error` as `cgp_error_anyhow::Error`.

## License

Licensed under the [Apache License, Version 2.0](https://github.com/contextgeneric/cgp/blob/main/LICENSE).
