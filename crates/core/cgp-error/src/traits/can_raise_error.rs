use cgp::component::{DefaultNamespace, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::has_error_type::HasErrorType;

/**
   The `CanRaiseError` trait is used to raise any concrete error type into
   an abstract error provided by [`HasErrorType`].
*/
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
