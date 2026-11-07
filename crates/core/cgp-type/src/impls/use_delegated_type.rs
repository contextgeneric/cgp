use core::marker::PhantomData;

use cgp_component::{DelegateComponent, WithProvider};
use cgp_macro::cgp_provider;

use crate::TypeProviderComponent;
use crate::traits::TypeProvider;

/// A provider that delegates an abstract type to a table.
///
/// See [`UseDelegatedType`](https://contextgeneric.dev/docs/reference/providers/use_delegated_type).
pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

/// A provider that wraps a delegated abstract type.
///
/// See [`WithDelegatedType`](https://contextgeneric.dev/docs/reference/providers/with_delegated_type).
pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Components, Type> TypeProvider<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
