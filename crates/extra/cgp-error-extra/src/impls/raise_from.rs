use cgp_core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use cgp_core::prelude::*;

#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for RaiseFrom
where
    Context: HasErrorType,
    Context::Error: From<E>,
{
    fn raise_error(e: E) -> Context::Error {
        e.into()
    }
}
