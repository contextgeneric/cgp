# cgp-error-std

`cgp-error-std` makes a boxed standard error, `Box<dyn core::error::Error + Send + Sync>`, the
abstract error type of a [CGP](https://contextgeneric.dev) context, and supplies the providers that
raise errors into it and add context to it. It depends only on `cgp` and `alloc`, so it suits a
`no_std` context that still wants an open-ended error type.

| Item | Wire it to | What it does |
|---|---|---|
| `UseBoxedStdError` | `ErrorTypeProviderComponent` | sets the context's `Error` to `cgp_error_std::Error` |
| `RaiseBoxedStdError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | boxes a standard error without formatting it, so `downcast_ref` still finds it; wraps a `Display` detail in a `WrapError` |
| `DebugBoxedStdError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises any `Debug` value as a `StringError` formatted with `{:?}`, and wraps a `Debug` detail in a `WrapError` |
| `DisplayBoxedStdError` | `ErrorRaiserComponent`, `ErrorWrapperComponent` | raises any `Display` value as a `StringError` formatted with `{}`, and wraps a `Display` detail in a `WrapError` |
| `Error` | — | the boxed error type itself |
| `StringError` | — | a standard error holding only a message |
| `WrapError` | — | a standard error holding a detail message and the error it wraps, returned as its `source` |

A context usually routes each source type to the provider that suits it:

```rust,ignore
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_std::{DisplayBoxedStdError, RaiseBoxedStdError, UseBoxedStdError};

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseBoxedStdError,
        @ErrorRaiserComponent.std::io::Error: RaiseBoxedStdError,
        @ErrorRaiserComponent.String: DisplayBoxedStdError,
        ErrorWrapperComponent: RaiseBoxedStdError,
    }
}

let error = App::raise_error(std::io::Error::other("disk full"));
let error = App::wrap_error(error, "while saving");
assert_eq!(format!("{error}"), "while saving");
assert_eq!(format!("{error:#}"), "while saving: disk full");
```

A `WrapError` prints its detail alone with `{}`, so a reporter that walks `source()` prints each
message once, and prints the whole chain with `{:#}` or `{:?}`. A `String` needs
`DisplayBoxedStdError` or `DebugBoxedStdError`, because it is not a standard error, and
`DebugBoxedStdError` prints it with quotes.
