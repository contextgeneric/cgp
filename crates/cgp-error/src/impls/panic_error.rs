use core::fmt::Debug;

use crate::traits::{ErrorRaiser, HasErrorType};

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
