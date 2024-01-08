use std::error::Error as StdError;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiser, ProvideErrorType};

pub type Error = Box<dyn StdError + Send + Sync + 'static>;

pub struct HandleErrorsWithStdError;

impl<Context> ProvideErrorType<Context> for HandleErrorsWithStdError
where
    Context: Async,
{
    type Error = Error;
}

impl<Context, E> ErrorRaiser<Context, E> for HandleErrorsWithStdError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}
