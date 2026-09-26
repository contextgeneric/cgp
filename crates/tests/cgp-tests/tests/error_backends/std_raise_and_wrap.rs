//! `RaiseBoxedStdError` boxes a standard error without formatting it, so the source is still found
//! by `downcast_ref`, and as a wrapper puts a `Display` detail in a `WrapError`.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-std/testing.md.

use std::io;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_std::{Error, RaiseBoxedStdError, UseBoxedStdError, WrapError};

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseBoxedStdError,
        ErrorRaiserComponent: RaiseBoxedStdError,
        ErrorWrapperComponent: RaiseBoxedStdError,
    }
}

check_components! {
    App {
        ErrorTypeProviderComponent,
        ErrorRaiserComponent: io::Error,
        ErrorWrapperComponent: [&'static str, String, u32],
    }
}

#[test]
fn test_std_raise_and_wrap() {
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
    assert_eq!(
        format!("{error:?}"),
        "while starting: while loading: no file"
    );

    let outer = error.downcast_ref::<WrapError>().unwrap();
    assert_eq!(outer.detail, "while starting");
}
