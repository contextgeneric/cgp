//! `RaiseEyreError` raises a standard error into `eyre::Report` without formatting it, and as a
//! wrapper adds a `'static` detail with `wrap_err`. No test in this target installs an eyre hook,
//! so building a report here also checks that the crate turns on eyre's `auto-install` feature.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-eyre/testing.md.

use std::io;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_eyre::{Error, RaiseEyreError, UseEyreError};

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseEyreError,
        ErrorRaiserComponent: RaiseEyreError,
        ErrorWrapperComponent: RaiseEyreError,
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
fn test_eyre_raise_and_wrap() {
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
    assert!(error.downcast_ref::<io::Error>().is_some());

    // The default handler prints the chain, then a `Location:` section, then a backtrace section
    // when `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` is set, so only the chain is matched exactly.
    // `eyre_location.rs` checks the location itself.
    let debug = format!("{error:?}");
    assert!(
        debug.starts_with("while starting\n\nCaused by:\n   0: while loading\n   1: no file"),
        "unexpected report: {debug}"
    );
}
