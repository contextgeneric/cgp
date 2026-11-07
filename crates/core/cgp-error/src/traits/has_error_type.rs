use core::fmt::Debug;

use cgp::component::DefaultNamespace;
use cgp_macro::cgp_type;

/// The component that gives a context one abstract Error type, so fallible generic code never names
/// a concrete error and the context chooses it by wiring.
///
/// See [`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type).
#[cgp_type]
#[prefix(@cgp.core.error in DefaultNamespace)]
pub trait HasErrorType {
    /// The context's error type.
    type Error: Debug;
}

/// The error type a context chooses.
pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
