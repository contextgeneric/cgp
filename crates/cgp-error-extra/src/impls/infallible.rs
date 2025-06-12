use core::convert::Infallible;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_core::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
