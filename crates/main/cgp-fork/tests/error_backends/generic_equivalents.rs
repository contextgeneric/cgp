//! The generic `UseType<anyhow::Error>` and `RaiseFrom` do the same job as `UseAnyhowError` and
//! `RaiseAnyhowError`'s raiser, since both convert through `From`. What a backend adds over the
//! generic providers is its wrappers and its formatting raisers. `RaiseFrom` also re-raises a value
//! that is already the context's error, through the reflexive `From<T> for T`, which the backend's
//! raiser cannot do because `anyhow::Error` is not itself a standard error.
//!
//! See cgp-knowledge-base-fork/projects/error/guides/choosing-a-backend.md.

use std::io;

use cgp_fork::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp_fork::extra::error::RaiseFrom;
use cgp_fork::prelude::*;

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<cgp_fork::anyhow::Error>,
        ErrorRaiserComponent: RaiseFrom,
    }
}

check_components! {
    App {
        ErrorRaiserComponent: [io::Error, cgp_fork::anyhow::Error],
    }
}

#[test]
fn test_generic_equivalents() {
    let error: cgp_fork::anyhow::Error = App::raise_error(io::Error::other("disk full"));
    assert!(error.downcast_ref::<io::Error>().is_some());
    assert_eq!(format!("{error}"), "disk full");

    let error: cgp_fork::anyhow::Error = App::raise_error(error);
    assert!(error.downcast_ref::<io::Error>().is_some());
}
