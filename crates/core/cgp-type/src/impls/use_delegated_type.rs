use core::marker::PhantomData;

use cgp_component::{DelegateComponent, IsProviderFor, WithProvider};
use cgp_macro::cgp_provider;

use crate::TypeProviderComponent;
use crate::traits::TypeProvider;

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Components, Type> TypeProvider<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
