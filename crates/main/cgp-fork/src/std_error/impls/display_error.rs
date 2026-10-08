use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt::Display;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use crate::core::prelude::*;
use crate::std_error::{StringError, WrapError};

/// Raises any `Display` value as a [`StringError`] formatted with `{}`, and wraps a `Display`
/// detail in a [`WrapError`] the same way. The original value is not kept.
pub struct DisplayBoxedStdError;

#[cgp_impl(DisplayBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::std_error::Error})]
impl<E> ErrorRaiser<E>
where
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(e.to_string()))
    }
}

#[cgp_impl(DisplayBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::std_error::Error})]
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
