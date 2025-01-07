use cgp_component::{DelegateComponent, HasComponents, UseDelegate};
use cgp_component_macro::cgp_component;

use crate::traits::has_error_type::HasErrorType;

/**
   Used for injecting external error types into [`Self::Error`](HasErrorType::Error).

   As an example, if `Context: CanRaiseError<ParseIntError>`, then we would be
   able to call `Context::raise_error(err)` for an error value
   [`err: ParseIntError`](core::num::ParseIntError) and get back
   a [`Context::Error`](HasErrorType::Error) value.
*/
#[cgp_component {
    provider: ErrorRaiser
}]
pub trait CanRaiseError<SourceError>: HasErrorType {
    fn raise_error(error: SourceError) -> Self::Error;
}

impl<Context, SourceError, Components, Delegate> ErrorRaiser<Context, SourceError>
    for UseDelegate<Components>
where
    Context: HasErrorType,
    Components: DelegateComponent<SourceError, Delegate = Delegate>,
    Delegate: ErrorRaiser<Context, SourceError>,
{
    fn raise_error(e: SourceError) -> Context::Error {
        Delegate::raise_error(e)
    }
}
