use alloc::boxed::Box;
use alloc::format;
use core::fmt::Debug;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp_core::prelude::*;

use crate::types::{Error, StringError};
use crate::WrapError;

pub struct DebugBoxedStdError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for DebugBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(format!("{e:?}")))
    }
}

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        Box::new(WrapError {
            detail: format!("{detail:?}"),
            source: error,
        })
    }
}
