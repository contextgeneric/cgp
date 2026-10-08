use alloc::boxed::Box;
use alloc::format;
use core::fmt::Debug;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use crate::core::prelude::*;
use crate::std_error::{StringError, WrapError};

/// Raises any `Debug` value as a [`StringError`] formatted with `{:?}`, and wraps a `Debug` detail
/// in a [`WrapError`] the same way. The original value is not kept.
pub struct DebugBoxedStdError;

#[cgp_impl(DebugBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::std_error::Error})]
impl<E> ErrorRaiser<E>
where
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(format!("{e:?}")))
    }
}

#[cgp_impl(DebugBoxedStdError)]
#[use_type(HasErrorType.{Error = crate::std_error::Error})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        Box::new(WrapError {
            detail: format!("{detail:?}"),
            source: error,
        })
    }
}
