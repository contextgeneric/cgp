use core::fmt::Debug;

use cgp_async::Async;
use cgp_component::{derive_component, DelegateComponent, HasComponents};

/**
   This is used for contexts to declare that they have a _unique_ `Self::Error` type.

   Although it is possible for each context to declare their own associated
   `Error` type, doing so may result in having multiple ambiguous `Self::Error` types,
   if there are multiple associated types with the same name in different traits.

   As a result, it is better for context traits to include `HasError` as their
   parent traits, so that multiple traits can all refer to the same abstract
   `Self::Error` type.
*/
#[derive_component(ErrorTypeComponent, ProvideErrorType<Context>)]
pub trait HasErrorType: Async {
    /**
       The `Error` associated type is also required to implement [`Debug`].

       This is to allow `Self::Error` to be used in calls like `.unwrap()`,
       as well as for simpler error logging.
    */
    type Error: Async + Debug;
}

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;

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
