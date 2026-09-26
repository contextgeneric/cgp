use alloc::format;
use core::fmt::Debug;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;
use eyre::eyre;

/// Raises any `Debug` value into [`eyre::Report`] as a message formatted with `{:?}`, and wraps a
/// `Debug` detail the same way. The original value is not kept.
pub struct DebugEyreError;

#[cgp_impl(DebugEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<E> ErrorRaiser<E>
where
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        eyre!("{e:?}")
    }
}

#[cgp_impl(DebugEyreError)]
#[use_type(HasErrorType.{Error = eyre::Report})]
impl<Detail> ErrorWrapper<Detail>
where
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        error.wrap_err(format!("{detail:?}"))
    }
}
