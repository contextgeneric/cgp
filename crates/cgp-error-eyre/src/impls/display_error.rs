use core::fmt::Display;

use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;
use eyre::{eyre, Error};

pub struct DisplayEyreError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayEyreError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        eyre!("{e}")
    }
}
