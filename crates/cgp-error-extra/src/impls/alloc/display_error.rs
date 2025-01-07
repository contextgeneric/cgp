use alloc::format;
use alloc::string::String;
use core::fmt::Display;

use cgp_error::{CanRaiseError, ErrorRaiser};

pub struct DisplayError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayError
where
    Context: CanRaiseError<String>,
    E: Display,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(format!("{e}"))
    }
}
