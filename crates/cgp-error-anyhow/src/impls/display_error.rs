use alloc::string::ToString;
use core::fmt::Display;

use anyhow::{anyhow, Error};
use cgp_core::error::{ErrorRaiser, ErrorWrapper};
use cgp_core::prelude::*;

pub struct DisplayAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{e}")
    }
}

impl<Context, Detail> ErrorWrapper<Context, Detail> for DisplayAnyhowError
where
    Context: HasErrorType<Error = Error>,
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(detail.to_string())
    }
}
