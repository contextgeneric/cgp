use alloc::format;
use core::fmt::Debug;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;
use eyre::{eyre, Error};

pub struct DebugEyreError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for DebugEyreError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        eyre!("{:?}", e)
    }
}

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugEyreError
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(format!("{detail:?}"))
    }
}
