use core::fmt::Debug;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_core::prelude::*;

pub struct PanicOnError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for PanicOnError
where
    Context: HasErrorType,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        panic!("{e:?}")
    }
}
