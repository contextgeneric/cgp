# cgp-error-extra

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) lets each context choose its error
type, and how each kind of error is raised into it. `cgp-error-extra` provides generic providers for
that choice: they raise an error by conversion, return it unchanged, discard a detail, or format it
into a message, and a context routes each source error to one of them:

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

Projects reach these providers through the `cgp` crate, which re-exports them as
`cgp::extra::error`, so they depend on [`cgp`](https://crates.io/crates/cgp) alone, as the example
does. The website's reference has a page for each of its main items:
[`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from),
[`ReturnError`](https://contextgeneric.dev/docs/reference/providers/error/return_error),
[`RaiseInfallible`](https://contextgeneric.dev/docs/reference/providers/error/raise_infallible),
[`PanicOnError`](https://contextgeneric.dev/docs/reference/providers/error/panic_on_error),
[`DiscardDetail`](https://contextgeneric.dev/docs/reference/providers/error/discard_detail),
[`DebugError`](https://contextgeneric.dev/docs/reference/providers/error/debug_error), and
[`DisplayError`](https://contextgeneric.dev/docs/reference/providers/error/display_error).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
