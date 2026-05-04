use cgp_component::*;
use cgp_field::types::*;
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

#[cgp_component {
    provider: ErrorWrapper,
    derive_delegate: UseDelegate<Detail>,
}]
#[use_namespace(@cgp.core.error.ErrorWrapperComponent)]
pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(error: Self::Error, detail: Detail) -> Self::Error;
}
