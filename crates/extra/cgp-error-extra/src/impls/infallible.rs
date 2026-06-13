use core::convert::Infallible;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
