use alloc::format;
use core::fmt::Debug;

use anyhow::anyhow;
use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

/// Raises any `Debug` value into [`anyhow::Error`] as a message formatted with `{:?}`, and wraps a
/// `Debug` detail the same way. The original value is not kept.
pub struct DebugAnyhowError;

#[cgp_impl(DebugAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<E> ErrorRaiser<E>
where
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{e:?}")
    }
}

#[cgp_impl(DebugAnyhowError)]
#[use_type(HasErrorType.{Error = anyhow::Error})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.context(format!("{detail:?}"))
    }
}
