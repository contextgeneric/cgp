use alloc::string::{String, ToString};
use core::fmt::Display;

use cgp_core::error::{
    CanRaiseError, CanWrapError, ErrorRaiser, ErrorRaiserComponent, ErrorWrapper,
    ErrorWrapperComponent,
};
use cgp_core::prelude::*;

pub struct DisplayError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for DisplayError
where
    Context: CanRaiseError<String>,
    E: Display,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(e.to_string())
    }
}

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DisplayError
where
    Context: CanWrapError<String>,
    Detail: Display,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Context::wrap_error(error, detail.to_string())
    }
}
