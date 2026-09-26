use cgp::component::{DefaultNamespace, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

/**
   The `CanWrapError` trait is used to attach a detail to an abstract error
   provided by [`HasErrorType`].
*/
#[cgp_component(ErrorWrapper)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<Detail>)]
#[use_type(HasErrorType.Error)]
pub trait CanWrapError<Detail> {
    /// `#[track_caller]` here applies to every provider impl and to the generated forwarding
    /// impls, so an error library that records `Location::caller()` sees the caller's line.
    #[track_caller]
    fn wrap_error(error: Error, detail: Detail) -> Error;
}
