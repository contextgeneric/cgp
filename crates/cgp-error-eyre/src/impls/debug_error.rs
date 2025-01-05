use core::fmt::Debug;

use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;
use eyre::{eyre, Error};

pub struct DebugEyreError;

impl<Context, E> ErrorRaiser<Context, E> for DebugEyreError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        eyre!("{:?}", e)
    }
}
