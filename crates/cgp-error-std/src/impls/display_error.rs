use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt::Display;

use cgp_core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent, HasErrorType,
};
use cgp_core::prelude::*;

use crate::types::{Error, StringError};
use crate::WrapError;

pub struct DisplayBoxedStdError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for DisplayBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(e.to_string()))
    }
}

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DisplayBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    Detail: Display,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        Box::new(WrapError {
            detail: detail.to_string(),
            source: error,
        })
    }
}
