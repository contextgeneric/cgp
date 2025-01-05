use core::fmt::Debug;

use crate::{CanRaiseError, ErrorRaiser};

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
