use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp_error::{CanRaiseError, ErrorRaiser};

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
