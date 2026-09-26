use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt::Display;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

use crate::{StringError, WrapError};

/// Raises any `Display` value as a [`StringError`] formatted with `{}`, and wraps a `Display`
/// detail in a [`WrapError`] the same way. The original value is not kept.
pub struct DisplayBoxedStdError;

#[cgp_impl(DisplayBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::Error})]
impl<E> ErrorRaiser<E>
where
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(e.to_string()))
    }
}

#[cgp_impl(DisplayBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::Error})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        Box::new(WrapError {
            detail: detail.to_string(),
            source: error,
        })
    }
}
