//! The wiring example in `cgp-error-eyre`'s README, copied verbatim so that CI compiles and runs it.
//! The README marks the block `ignore` because in the crate's own doctests `cgp` names `cgp-core`;
//! keep the two in sync when either changes.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-eyre/testing.md.

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

#[test]
fn test_readme_eyre() {
    let error = App::raise_error(std::io::Error::other("disk full"));
    let error = App::wrap_error(error, "while saving");
    assert_eq!(format!("{error:#}"), "while saving: disk full");
}
