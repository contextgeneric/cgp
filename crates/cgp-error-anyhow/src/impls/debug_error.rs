use core::fmt::Debug;

use anyhow::{anyhow, Error};
use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;

pub struct DebugAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for DebugAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{:?}", e)
    }
}
