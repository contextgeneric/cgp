use core::error::Error as StdError;

use anyhow::Error;
use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;

pub struct RaiseAnyhowError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseAnyhowError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}
