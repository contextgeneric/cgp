use alloc::string::ToString;
use core::fmt::Display;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;
use eyre::eyre;

/// Raises any `Display` value into [`eyre::Report`] as a message formatted with `{}`, and wraps a
/// `Display` detail the same way. The original value is not kept.
pub struct DisplayEyreError;

#[cgp_impl(DisplayEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<E> ErrorRaiser<E>
where
    E: Display,
{
    fn raise_error(e: E) -> Error {
        eyre!("{e}")
    }
}

#[cgp_impl(DisplayEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(detail.to_string())
    }
}
