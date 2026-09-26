use alloc::boxed::Box;
use alloc::string::ToString;
use core::error::Error as StdError;
use core::fmt::Display;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

use crate::WrapError;

/// Boxes a standard error without formatting it, so the source stays available to `downcast_ref`
/// and to the error chain, and wraps a `Display` detail in a [`WrapError`].
pub struct RaiseBoxedStdError;

#[cgp_impl(RaiseBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::Error})]
impl<E> ErrorRaiser<E>
where
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        Box::new(e)
    }
}

#[cgp_impl(RaiseBoxedStdError)]
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
