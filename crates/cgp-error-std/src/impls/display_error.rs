use alloc::boxed::Box;
use alloc::format;
use core::fmt::Display;

use cgp_core::error::{ErrorRaiser, HasErrorType};

use crate::types::{Error, StringError};

pub struct DisplayBoxedStdError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(format!("{e}")))
    }
}
