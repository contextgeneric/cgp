//! `DebugBoxedStdError` and `DisplayBoxedStdError` raise any `Debug` or `Display` value as a
//! `StringError` holding the formatted message, and wrap a detail in a `WrapError` the same way.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-std/testing.md.

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_std::{DebugBoxedStdError, DisplayBoxedStdError, StringError, UseBoxedStdError};

#[derive(Debug)]
pub struct Rejected {
    pub code: u32,
}

pub struct App;

delegate_components! {
    App {
        open { ErrorRaiserComponent, ErrorWrapperComponent };

        ErrorTypeProviderComponent: UseBoxedStdError,
        @ErrorRaiserComponent.Rejected: DebugBoxedStdError,
        @ErrorRaiserComponent.String: DisplayBoxedStdError,
        @ErrorWrapperComponent.String: DisplayBoxedStdError,
        @ErrorWrapperComponent.u32: DebugBoxedStdError,
    }
}

check_components! {
    App {
        ErrorRaiserComponent: [Rejected, String],
        ErrorWrapperComponent: [String, u32],
    }
}

#[test]
fn test_std_formatting() {
    let error = App::raise_error(Rejected { code: 7 });
    assert_eq!(
        error.downcast_ref::<StringError>().unwrap().message,
        "Rejected { code: 7 }"
    );

    let error = App::raise_error(String::from("plain message"));
    let error = App::wrap_error(error, 42u32);
    let error = App::wrap_error(error, String::from("while checking"));
    assert_eq!(format!("{error}"), "while checking");
    assert_eq!(format!("{error:#}"), "while checking: 42: plain message");
}
