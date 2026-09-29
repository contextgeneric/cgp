use cgp_fork::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_fork::prelude::*;

#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for ReturnError
where
    Context: HasErrorType<Error = E>,
{
    fn raise_error(e: E) -> E {
        e
    }
}
