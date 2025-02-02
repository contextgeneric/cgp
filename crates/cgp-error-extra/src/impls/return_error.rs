use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent};
use cgp_core::prelude::*;

pub struct ReturnError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for ReturnError
where
    Context: HasErrorType<Error = E>,
{
    fn raise_error(e: E) -> E {
        e
    }
}
