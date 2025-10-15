use core::error::Error as StdError;
use core::fmt::Display;

use anyhow::Error;
use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;

pub struct RaiseAnyhowError;

#[cgp_impl(RaiseAnyhowError)]
impl<Context, E> ErrorRaiser<E> for Context
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

#[cgp_impl(RaiseAnyhowError)]
impl<Context, Detail> ErrorWrapper<Detail> for Context
where
    Context: HasErrorType<Error = Error>,
    Detail: Display + Send + Sync + 'static,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(detail)
    }
}
