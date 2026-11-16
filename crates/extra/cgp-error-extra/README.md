# cgp-error-extra

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. A context chooses, the same way, how each kind of error
becomes its own error type, and `cgp-error-extra` provides generic providers for that choice. They
raise an error by conversion, return it unchanged, discard a detail, or format it into a message,
and a context routes each source error to one of them:

```rust,ignore
use core::num::ParseIntError;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::{DebugError, RaiseFrom};
use cgp::prelude::*;

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<String>,
        @ErrorRaiserComponent.String: RaiseFrom,
        @ErrorRaiserComponent.ParseIntError: DebugError,
    }
}

fn main() {
    let error = "http".parse::<u16>().map_err(App::raise_error).unwrap_err();
    assert!(error.contains("InvalidDigit"));
}
```

`DebugError` and `DisplayError` format an error into a `String`, so they need an allocator and the
`alloc` feature, which this example turns on with
`cgp = { version = "0.8.0", features = ["alloc"] }`.

The website's reference covers its main items:
[`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from),
[`ReturnError`](https://contextgeneric.dev/docs/reference/providers/error/return_error),
[`RaiseInfallible`](https://contextgeneric.dev/docs/reference/providers/error/raise_infallible),
[`PanicOnError`](https://contextgeneric.dev/docs/reference/providers/error/panic_on_error),
[`DiscardDetail`](https://contextgeneric.dev/docs/reference/providers/error/discard_detail),
[`DebugError`](https://contextgeneric.dev/docs/reference/providers/error/debug_error), and
[`DisplayError`](https://contextgeneric.dev/docs/reference/providers/error/display_error).

The `cgp` crate re-exports it as `cgp::extra::error`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.