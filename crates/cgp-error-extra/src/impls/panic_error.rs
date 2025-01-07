use core::fmt::Debug;

use cgp_error::{ErrorRaiser, HasErrorType};

pub struct PanicOnError;

impl<Context, E> ErrorRaiser<Context, E> for PanicOnError
where
    Context: HasErrorType,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        panic!("{e:?}")
    }
}
