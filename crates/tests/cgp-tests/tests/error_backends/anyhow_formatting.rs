//! `DebugAnyhowError` and `DisplayAnyhowError` raise any `Debug` or `Display` value, including a
//! `String` that is not a standard error, by formatting it into a new anyhow message, and wrap a
//! detail the same way. The original value is not kept, and `{:?}` quotes a string.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-anyhow/testing.md.

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{DebugAnyhowError, DisplayAnyhowError, UseAnyhowError};

#[derive(Debug)]
pub struct Rejected {
    pub code: u32,
}

pub struct App;

delegate_components! {
    App {
        open { ErrorRaiserComponent, ErrorWrapperComponent };

        ErrorTypeProviderComponent: UseAnyhowError,
        @ErrorRaiserComponent.Rejected: DebugAnyhowError,
        @ErrorRaiserComponent.String: DisplayAnyhowError,
        @ErrorRaiserComponent.&'static str: DebugAnyhowError,
        @ErrorWrapperComponent.String: DisplayAnyhowError,
        @ErrorWrapperComponent.u32: DebugAnyhowError,
    }
}

check_components! {
    App {
        ErrorRaiserComponent: [Rejected, String, &'static str],
        ErrorWrapperComponent: [String, u32],
    }
}

#[test]
fn test_anyhow_formatting() {
    let error = App::raise_error(Rejected { code: 7 });
    assert_eq!(format!("{error}"), "Rejected { code: 7 }");
    // The report holds only the formatted message, with no source behind it.
    assert_eq!(error.chain().count(), 1);

    let error = App::raise_error(String::from("plain message"));
    assert_eq!(format!("{error}"), "plain message");

    let error = App::raise_error("quoted message");
    assert_eq!(format!("{error}"), "\"quoted message\"");

    let error = App::wrap_error(error, 42u32);
    let error = App::wrap_error(error, String::from("while checking"));
    assert_eq!(
        format!("{error:#}"),
        "while checking: 42: \"quoted message\""
    );
}
