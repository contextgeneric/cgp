use cgp_async::Async;
use cgp_component::{derive_component, DelegateComponent, HasComponents};

use crate::has_error_type::HasErrorType;

/**
   Used for injecting external error types into [`Self::Error`](HasErrorType::Error).

   As an example, if `Context: CanRaiseError<ParseIntError>`, then we would be
   able to call `Context::raise_error(err)` for an error value
   [`err: ParseIntError`](core::num::ParseIntError) and get back
   a [`Context::Error`](HasErrorType::Error) value.
*/
#[derive_component(ErrorRaiserComponent, ErrorRaiser<Context>)]
pub trait CanRaiseError<E>: HasErrorType {
    fn raise_error(e: E) -> Self::Error;
}
