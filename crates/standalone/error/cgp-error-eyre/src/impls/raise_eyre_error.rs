use core::error::Error as StdError;
use core::fmt::Display;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

/// Raises a standard error into [`eyre::Report`] without formatting it, so the source stays
/// available to `downcast_ref` and to the error chain, and wraps a detail with `wrap_err`. The
/// report records the location that called `raise_error`.
pub struct RaiseEyreError;

#[cgp_impl(RaiseEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<E> ErrorRaiser<E>
where
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

#[cgp_impl(RaiseEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Display + Send + Sync + 'static,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(detail)
    }
}
