use core::error::Error as StdError;

use cgp_core::error::ErrorRaiser;
use cgp_core::prelude::*;
use eyre::Error;

pub struct RaiseEyreError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseEyreError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}
