use core::marker::PhantomData;

use cgp_component::{DelegateComponent, WithProvider};
use cgp_macro::cgp_provider;

use crate::TypeProviderComponent;
use crate::traits::TypeProvider;

/// The type provider that resolves an abstract type by looking its tag up in a delegate_components!
/// table, instead of fixing it to one concrete type.
///
/// See [`UseDelegatedType`](https://contextgeneric.dev/docs/reference/providers/use_delegated_type)
/// .
pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

/// The alias WithProvider`<UseDelegatedType<Table>`>: several `#[cgp_type]` components answered
/// from one table keyed by their component markers.
///
/// See
/// [`WithDelegatedType`](https://contextgeneric.dev/docs/reference/providers/with_delegated_type) .
pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Components, Type> TypeProvider<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
