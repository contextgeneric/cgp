use cgp::macro_prelude::*;
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

#[cgp_component {
    provider: ErrorWrapper,
    derive_delegate: UseDelegate<Detail>,
}]
#[prefix(@cgp.core.error)]
pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(error: Self::Error, detail: Detail) -> Self::Error;
}
