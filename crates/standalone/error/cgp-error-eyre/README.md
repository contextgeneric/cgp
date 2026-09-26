# cgp-error-eyre

`cgp-error-eyre` makes [`eyre::Report`](https://docs.rs/eyre) the abstract error type of a
[CGP](https://contextgeneric.dev) context, and supplies the providers that raise errors into it and
add context to it. Code written against `HasErrorType`, `CanRaiseError`, and `CanWrapError` stays
generic; only the context's wiring names eyre.

| Provider | Wire it to | What it does |
|---|---|---|
| `UseEyreError` | `ErrorTypeProviderComponent` | sets the context's `Error` to `eyre::Report` |
| `RaiseEyreError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises a standard error without formatting it, so `downcast_ref` still finds it; wraps a `Display + Send + Sync + 'static` detail with `wrap_err` |
| `DebugEyreError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises or wraps any `Debug` value as a message formatted with `{:?}` |
| `DisplayEyreError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises or wraps any `Display` value as a message formatted with `{}` |

A context usually routes each source type to the provider that suits it:

```rust,ignore
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_eyre::{DisplayEyreError, RaiseEyreError, UseEyreError};

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseEyreError,
        @ErrorRaiserComponent.std::io::Error: RaiseEyreError,
        @ErrorRaiserComponent.String: DisplayEyreError,
        ErrorWrapperComponent: RaiseEyreError,
    }
}

let error = App::raise_error(std::io::Error::other("disk full"));
let error = App::wrap_error(error, "while saving");
assert_eq!(format!("{error:#}"), "while saving: disk full");
```

A `String` needs `DisplayEyreError` or `DebugEyreError`, because it is not a standard error.
`DebugEyreError` prints it with quotes.

The crate enables eyre's `auto-install` feature, so eyre's default report handler is installed the
first time a report is built. To use another handler, such as `color-eyre`, install it with
`eyre::set_hook` before the first error is raised; once a report exists, `set_hook` returns an
error. The crate leaves eyre's `track-caller` feature off, because every report is built inside one
of its providers and the recorded location would name that line rather than the caller. eyre
requires `std`, so this crate does too.

The crate re-exports `eyre::Error`, eyre's alias for `Report`, as `cgp_error_eyre::Error`.
