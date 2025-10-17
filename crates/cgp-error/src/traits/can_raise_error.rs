use cgp_component::{DelegateComponent, IsProviderFor, UseContext, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::has_error_type::HasErrorType;

/**
   The `CanRaiseError` trait is used to raise any concrete error type into
   an abstract error provided by [`HasErrorType`].
*/
#[cgp_component {
    provider: ErrorRaiser,
    derive_delegate: UseDelegate<SourceError>,
}]
pub trait CanRaiseError<SourceError>: HasErrorType {
    fn raise_error(error: SourceError) -> Self::Error;
}
