use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp_error::{CanRaiseError, CanWrapError, ErrorRaiser, ErrorWrapper};

pub struct DebugError;

impl<Context, E> ErrorRaiser<Context, E> for DebugError
where
    Context: CanRaiseError<String>,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(format!("{e:?}"))
    }
}

impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugError
where
    Context: CanWrapError<String>,
    Detail: Debug,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Context::wrap_error(error, format!("{detail:?}"))
    }
}
