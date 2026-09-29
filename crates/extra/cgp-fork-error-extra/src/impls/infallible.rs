use core::convert::Infallible;

use cgp_fork::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_fork::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
