use core::convert::Infallible;

use crate::traits::{ErrorRaiser, HasErrorType};

pub struct RaiseInfallible;

impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
