use cgp::component::{DefaultNamespace, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

#[cgp_component(ErrorWrapper)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<Detail>)]
#[use_type(HasErrorType::Error)]
pub trait CanWrapError<Detail> {
    fn wrap_error(error: Error, detail: Detail) -> Error;
}
