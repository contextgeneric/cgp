//! With eyre's `track-caller` feature on, a report records where it was built. `raise_error` and
//! `wrap_error` are `#[track_caller]` in their trait declarations, which carries through every
//! provider impl and CGP's generated forwarding impls, so the recorded location is the line that
//! called `raise_error`: through plain, `open`, and namespace-path wiring alike, for each eyre
//! provider and for the generic `RaiseFrom`, and unchanged by wrapping.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-eyre/testing.md.

use std::io;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::extra::error::RaiseFrom;
use cgp::prelude::*;
use cgp_error_eyre::{DebugEyreError, DisplayEyreError, Error, RaiseEyreError, UseEyreError};

#[derive(Debug)]
pub struct Rejected;

pub struct PlainApp;

delegate_components! {
    PlainApp {
        ErrorTypeProviderComponent: UseEyreError,
        [ErrorRaiserComponent, ErrorWrapperComponent]: RaiseEyreError,
    }
}

pub struct OpenApp;

delegate_components! {
    OpenApp {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseEyreError,
        @ErrorRaiserComponent.io::Error: RaiseEyreError,
        @ErrorRaiserComponent.String: DisplayEyreError,
        @ErrorRaiserComponent.Rejected: DebugEyreError,
    }
}

pub struct NamespaceApp;

delegate_components! {
    NamespaceApp {
        namespace DefaultNamespace;

        @cgp.core.error.ErrorTypeProviderComponent: UseEyreError,
        @cgp.core.error.ErrorRaiserComponent.io::Error: RaiseEyreError,
    }
}

pub struct GenericApp;

delegate_components! {
    GenericApp {
        ErrorTypeProviderComponent: UseType<Error>,
        ErrorRaiserComponent: RaiseFrom,
    }
}

/// The `file:line` the default handler prints under `Location:`, without the column.
fn location(error: &Error) -> String {
    let debug = format!("{error:?}");
    let located = debug
        .split("Location:\n")
        .nth(1)
        .unwrap_or_else(|| panic!("no location in report: {debug}"));
    let location = located.lines().next().unwrap().trim();
    location.rsplit_once(':').unwrap().0.to_owned()
}

fn here(line: u32) -> String {
    format!("{}:{line}", file!())
}

#[test]
fn test_eyre_location() {
    let line = line!() + 1;
    let error = PlainApp::raise_error(io::Error::other("disk full"));
    assert_eq!(location(&error), here(line));

    // Wrapping keeps the location where the report was first built.
    let error = PlainApp::wrap_error(error, "while saving");
    assert_eq!(location(&error), here(line));

    let line = line!() + 1;
    let error = OpenApp::raise_error(io::Error::other("disk full"));
    assert_eq!(location(&error), here(line));

    let line = line!() + 1;
    let error = OpenApp::raise_error(String::from("bad input"));
    assert_eq!(location(&error), here(line));

    let line = line!() + 1;
    let error = OpenApp::raise_error(Rejected);
    assert_eq!(location(&error), here(line));

    let line = line!() + 1;
    let error = NamespaceApp::raise_error(io::Error::other("disk full"));
    assert_eq!(location(&error), here(line));

    // The generic `RaiseFrom` converts with `Into::into`, which is `#[track_caller]` too.
    let line = line!() + 1;
    let error = GenericApp::raise_error(io::Error::other("disk full"));
    assert_eq!(location(&error), here(line));
}
