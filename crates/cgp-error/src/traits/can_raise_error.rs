use cgp_component::{DelegateComponent, IsProviderFor, RedirectLookup, UseContext, UseDelegate};
use cgp_macro::{cgp_component, cgp_impl};

use crate::traits::has_error_type::HasErrorType;

/**
   The `CanRaiseError` trait is used to raise any concrete error type into
   an abstract error provided by [`HasErrorType`].
*/
#[cgp_component {
    provider: ErrorRaiser,
    derive_delegate: UseDelegate<SourceError>,
}]
pub trait CanRaiseError<SourceError>: HasErrorType {
    fn raise_error(error: SourceError) -> Self::Error;
}

// #[cgp_impl(RedirectLookup<Key, Components>)]
// #[use_type(HasErrorType::Error)]
// #[use_provider(Components::Delegate: ErrorRaiser<E>)]
// impl<Key, Components, E> ErrorRaiser<E>
// where
//     Components: DelegateComponent<Key>,
// {
//     fn raise_error(error: E) -> Error {
//         Components::Delegate::raise_error(error)
//     }
// }
