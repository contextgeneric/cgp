use alloc::format;
use core::fmt::Debug;

use anyhow::{anyhow, Error};
use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;

pub struct DebugAnyhowError;

#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for DebugAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{:?}", e)
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugAnyhowError
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(format!("{detail:?}"))
    }
}
