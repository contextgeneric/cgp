//! `DebugEyreError` and `DisplayEyreError` raise any `Debug` or `Display` value by formatting it
//! into a new eyre report, and wrap a detail the same way. The original value is not kept.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-eyre/testing.md.

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_eyre::{DebugEyreError, DisplayEyreError, UseEyreError};

#[derive(Debug)]
pub struct Rejected {
    pub code: u32,
}

pub struct App;

delegate_components! {
    App {
        open { ErrorRaiserComponent, ErrorWrapperComponent };

        ErrorTypeProviderComponent: UseEyreError,
        @ErrorRaiserComponent.Rejected: DebugEyreError,
        @ErrorRaiserComponent.String: DisplayEyreError,
        @ErrorWrapperComponent.String: DisplayEyreError,
        @ErrorWrapperComponent.u32: DebugEyreError,
    }
}

check_components! {
    App {
        ErrorRaiserComponent: [Rejected, String],
        ErrorWrapperComponent: [String, u32],
    }
}

#[test]
fn test_eyre_formatting() {
    let error = App::raise_error(Rejected { code: 7 });
    assert_eq!(format!("{error}"), "Rejected { code: 7 }");
    // The report holds only the formatted message, with no source behind it.
    assert_eq!(error.chain().count(), 1);

    let error = App::raise_error(String::from("plain message"));
    let error = App::wrap_error(error, 42u32);
    let error = App::wrap_error(error, String::from("while checking"));
    assert_eq!(format!("{error:#}"), "while checking: 42: plain message");
}
