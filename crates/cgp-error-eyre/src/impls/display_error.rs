use alloc::string::ToString;
use core::fmt::Display;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;
use eyre::{Error, eyre};

pub struct DisplayEyreError;

#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for DisplayEyreError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        eyre!("{e}")
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DisplayEyreError
where
    Context: HasErrorType<Error = Error>,
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(detail.to_string())
    }
}
