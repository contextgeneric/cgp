use core::error::Error as StdError;
use core::fmt::Display;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;
use eyre::Error;

pub struct RaiseEyreError;

#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for RaiseEyreError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for RaiseEyreError
where
    Context: HasErrorType<Error = Error>,
    Detail: Display + Send + Sync + 'static,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(detail)
    }
}
