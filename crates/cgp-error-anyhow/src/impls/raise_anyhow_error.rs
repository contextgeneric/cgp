use core::error::Error as StdError;
use core::fmt::Display;

use anyhow::Error;
use cgp_core::error::{ErrorRaiser, ErrorWrapper};
use cgp_core::prelude::*;

pub struct RaiseAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

impl<Context, Detail> ErrorWrapper<Context, Detail> for RaiseAnyhowError
where
    Context: HasErrorType<Error = Error>,
    Detail: Display + Send + Sync + 'static,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(detail)
    }
}
