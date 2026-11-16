# cgp-error

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) lets each context choose how a
trait is implemented, and its error handling extends that choice to the error type itself.
`cgp-error` provides the components behind it: the abstract error type, and the components that
raise errors into it and wrap them with detail. Generic code raises an error without knowing its
type, and each context decides what it becomes:

```rust,ignore
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::prelude::*;

pub fn parse_port<Context>(input: &str) -> Result<u16, Context::Error>
where
    Context: CanRaiseError<String>,
{
    input.parse().map_err(|_| Context::raise_error(format!("not a port: {input}")))
}

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<String>,
        @ErrorRaiserComponent.String: RaiseFrom,
    }
}

fn main() {
    assert_eq!(parse_port::<App>("80"), Ok(80));
    assert_eq!(parse_port::<App>("http"), Err("not a port: http".to_owned()));
}
```

The website's reference covers its main items:
[`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type),
[`CanRaiseError`](https://contextgeneric.dev/docs/reference/components/can_raise_error), and
[`CanWrapError`](https://contextgeneric.dev/docs/reference/components/can_wrap_error).

The `cgp` crate re-exports it as `cgp::core::error`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.