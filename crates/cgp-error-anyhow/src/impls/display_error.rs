use core::fmt::Display;

use anyhow::{anyhow, Error};
use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;

pub struct DisplayAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{e}")
    }
}
