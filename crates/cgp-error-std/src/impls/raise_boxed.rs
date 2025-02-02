use core::error::Error as StdError;

use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent};
use cgp_core::prelude::*;

use crate::types::Error;

pub struct RaiseBoxedStdError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseBoxedStdError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}
