use core::fmt::Debug;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use crate::core::prelude::*;

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
