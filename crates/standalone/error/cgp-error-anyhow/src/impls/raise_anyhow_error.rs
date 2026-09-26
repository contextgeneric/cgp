use core::error::Error as StdError;
use core::fmt::Display;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

/// Raises a standard error into [`anyhow::Error`] without formatting it, so the source stays
/// available to `downcast_ref` and to the error chain, and wraps a detail as anyhow context.
pub struct RaiseAnyhowError;

#[cgp_impl(RaiseAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<E> ErrorRaiser<E>
where
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

#[cgp_impl(RaiseAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Display + Send + Sync + 'static,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(detail)
    }
}
