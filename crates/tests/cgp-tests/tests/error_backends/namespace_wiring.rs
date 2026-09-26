//! A context that joins `DefaultNamespace` wires a backend through the full paths the error
//! components register under, `@cgp.core.error.*`, dispatching raisers per source type.
//!
//! See cgp-knowledge-base/projects/error/guides/choosing-a-backend.md.

use std::io;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{DisplayAnyhowError, RaiseAnyhowError, UseAnyhowError};

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        @cgp.core.error.ErrorTypeProviderComponent: UseAnyhowError,
        @cgp.core.error.ErrorRaiserComponent.io::Error: RaiseAnyhowError,
        @cgp.core.error.ErrorRaiserComponent.String: DisplayAnyhowError,
        @cgp.core.error.ErrorWrapperComponent.&'static str: RaiseAnyhowError,
    }
}

check_components! {
    App {
        ErrorTypeProviderComponent,
        ErrorRaiserComponent: [io::Error, String],
        ErrorWrapperComponent: &'static str,
    }
}

#[test]
fn test_namespace_wiring() {
    let error = App::raise_error(io::Error::other("disk full"));
    let error = App::wrap_error(error, "while saving");
    assert_eq!(format!("{error:#}"), "while saving: disk full");

    let error = App::raise_error(String::from("bad input"));
    assert_eq!(format!("{error}"), "bad input");
}
