use core::convert::Infallible;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use crate::core::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorRaiser<Context, Infallible> for RaiseInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
