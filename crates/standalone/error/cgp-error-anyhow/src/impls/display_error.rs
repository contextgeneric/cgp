use alloc::string::ToString;
use core::fmt::Display;

use anyhow::anyhow;
use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

/// Raises any `Display` value into [`anyhow::Error`] as a message formatted with `{}`, and wraps a
/// `Display` detail the same way. The original value is not kept.
pub struct DisplayAnyhowError;

#[cgp_impl(DisplayAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<E> ErrorRaiser<E>
where
    E: Display,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{e}")
    }
}

#[cgp_impl(DisplayAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(detail.to_string())
    }
}
