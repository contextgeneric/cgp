//! One provider written against `CanRaiseError` and `CanWrapError` runs unchanged on three
//! contexts that wire the three backends, and produces the same message from each.
//!
//! See cgp-knowledge-base-fork/projects/error/architecture.md.

use core::fmt::Display;
use core::num::ParseIntError;

use cgp_fork::core::error::{
    ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;

#[cgp_component(PortParser)]
#[use_type(HasErrorType.Error)]
pub trait CanParsePort {
    fn parse_port(&self, raw: &str) -> Result<u16, Error>;
}

#[cgp_impl(new ParseWithContext)]
#[uses(CanRaiseError<ParseIntError>, CanWrapError<String>)]
#[use_type(HasErrorType.Error)]
impl PortParser {
    fn parse_port(&self, raw: &str) -> Result<u16, Error> {
        raw.parse::<u16>()
            .map_err(Self::raise_error)
            .map_err(|e| Self::wrap_error(e, format!("invalid port {raw:?}")))
    }
}

macro_rules! backend_app {
    ($app:ident, $use:ty, $raise:ty) => {
        pub struct $app;

        delegate_components! {
            $app {
                ErrorTypeProviderComponent: $use,
                [ErrorRaiserComponent, ErrorWrapperComponent]: $raise,
                PortParserComponent: ParseWithContext,
            }
        }

        check_components! {
            $app {
                PortParserComponent,
            }
        }
    };
}

backend_app!(
    AnyhowApp,
    cgp_fork::anyhow::UseAnyhowError,
    cgp_fork::anyhow::RaiseAnyhowError
);
backend_app!(
    EyreApp,
    cgp_fork::eyre::UseEyreError,
    cgp_fork::eyre::RaiseEyreError
);
backend_app!(
    StdApp,
    cgp_fork::std_error::UseBoxedStdError,
    cgp_fork::std_error::RaiseBoxedStdError
);

fn assert_parses<App>(app: &App)
where
    App: CanParsePort,
    App::Error: Display,
{
    assert_eq!(app.parse_port("8080").ok(), Some(8080));

    let error = app.parse_port("80a").err().unwrap();
    assert_eq!(format!("{error}"), "invalid port \"80a\"");
    assert_eq!(
        format!("{error:#}"),
        "invalid port \"80a\": invalid digit found in string"
    );
}

#[test]
fn test_swapping_backends() {
    assert_parses(&AnyhowApp);
    assert_parses(&EyreApp);
    assert_parses(&StdApp);
}
