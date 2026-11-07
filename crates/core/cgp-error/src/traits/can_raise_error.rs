use cgp::component::{DefaultNamespace, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::has_error_type::HasErrorType;

/// The component that converts a concrete source error into the context's abstract error type,
/// dispatching per source error to the provider the context wires.
///
/// See [`CanRaiseError`](https://contextgeneric.dev/docs/reference/components/can_raise_error).
#[cgp_component(ErrorRaiser)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<SourceError>)]
#[use_type(HasErrorType.Error)]
pub trait CanRaiseError<SourceError> {
    /// `#[track_caller]` here applies to every provider impl and to the generated forwarding
    /// impls, so an error library that records `Location::caller()` sees the caller's line.
    #[track_caller]
    fn raise_error(error: SourceError) -> Error;
}
