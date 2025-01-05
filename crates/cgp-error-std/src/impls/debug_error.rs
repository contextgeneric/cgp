use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::format;
use cgp_core::error::{ErrorRaiser, HasErrorType};

use crate::types::{Error, StringError};

pub struct DebugBoxedStdError;

impl<Context, E> ErrorRaiser<Context, E> for DebugBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        Box::new(StringError::from(format!("{e:?}")))
    }
}
