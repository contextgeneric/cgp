//! `RaiseAnyhowError` raises a standard error into `anyhow::Error` without formatting it, so the
//! source is still found by `downcast_ref`, and as a wrapper it adds a `'static` detail as anyhow
//! context: `{}` prints the outermost detail and `{:#}` the whole chain.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-anyhow/testing.md.

use std::io;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{Error, RaiseAnyhowError, UseAnyhowError};

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseAnyhowError,
        ErrorRaiserComponent: RaiseAnyhowError,
        ErrorWrapperComponent: RaiseAnyhowError,
    }
}

check_components! {
    App {
        ErrorTypeProviderComponent,
        ErrorRaiserComponent: io::Error,
        ErrorWrapperComponent: [&'static str, String],
    }
}

#[test]
fn test_anyhow_raise_and_wrap() {
    let error: Error = App::raise_error(io::Error::new(io::ErrorKind::NotFound, "no file"));
    assert_eq!(
        error.downcast_ref::<io::Error>().map(io::Error::kind),
        Some(io::ErrorKind::NotFound)
    );

    let error = App::wrap_error(error, "while loading");
    let error = App::wrap_error(error, String::from("while starting"));

    assert_eq!(format!("{error}"), "while starting");
    assert_eq!(
        format!("{error:#}"),
        "while starting: while loading: no file"
    );

    // Wrapping keeps the original error reachable through the chain.
    assert!(error.downcast_ref::<io::Error>().is_some());
}
