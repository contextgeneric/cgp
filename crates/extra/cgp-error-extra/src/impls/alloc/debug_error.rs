use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use cgp::prelude::*;

/// Raises an error by formatting its source with `Debug`.
///
/// See [`DebugError`](https://contextgeneric.dev/docs/reference/providers/error/debug_error).
pub struct DebugError;

#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for DebugError
where
    Context: CanRaiseError<String>,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(format!("{e:?}"))
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugError
where
    Context: CanWrapError<String>,
    Detail: Debug,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Context::wrap_error(error, format!("{detail:?}"))
    }
}
