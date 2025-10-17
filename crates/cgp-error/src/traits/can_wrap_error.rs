use cgp_component::{DelegateComponent, IsProviderFor, UseContext, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

#[cgp_component {
    provider: ErrorWrapper,
    derive_delegate: UseDelegate<Detail>,
}]
pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(error: Self::Error, detail: Detail) -> Self::Error;
}
