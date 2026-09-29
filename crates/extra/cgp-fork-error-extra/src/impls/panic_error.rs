use core::fmt::Debug;

use cgp_fork::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_fork::prelude::*;

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
