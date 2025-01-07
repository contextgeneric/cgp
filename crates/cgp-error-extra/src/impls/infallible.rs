use core::convert::Infallible;

use cgp_error::{ErrorRaiser, HasErrorType};

pub struct RaiseInfallible;

impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
