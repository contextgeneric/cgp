use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_core::prelude::*;

pub struct RaiseFrom;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseFrom
where
    Context: HasErrorType,
    Context::Error: From<E>,
{
    fn raise_error(e: E) -> Context::Error {
        e.into()
    }
}
