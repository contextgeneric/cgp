use core::marker::PhantomData;

use cgp_component::WithProvider;
use cgp_macro::cgp_provider;

use crate::TypeProviderComponent;
use crate::traits::TypeProvider;

/// The provider that sets an abstract-type component to a concrete type, so a context chooses its
/// error, scalar, or runtime type with one wiring line.
///
/// See [`UseType`](https://contextgeneric.dev/docs/reference/providers/use_type).
pub struct UseType<Type>(pub PhantomData<Type>);

/// The alias WithProvider`<UseType<T>`>, which binds an abstract-type component to a concrete type
/// exactly as the plain UseType`<T>` does.
///
/// See [`WithType`](https://contextgeneric.dev/docs/reference/providers/with_type).
pub type WithType<Type> = WithProvider<UseType<Type>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Type> TypeProvider<Context, Tag> for UseType<Type> {
    type Type = Type;
}
