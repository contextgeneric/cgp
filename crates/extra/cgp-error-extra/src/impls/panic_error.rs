use core::fmt::Debug;

use cgp::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for PanicOnError
where
    Context: HasErrorType,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        panic!("{e:?}")
    }
}
