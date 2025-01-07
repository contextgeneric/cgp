use core::fmt::Debug;

use alloc::format;
use anyhow::{anyhow, Error};
use cgp_core::error::{ErrorRaiser, ErrorWrapper};
use cgp_core::prelude::*;

pub struct DebugAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for DebugAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{:?}", e)
    }
}

impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugAnyhowError
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(format!("{detail:?}"))
    }
}
