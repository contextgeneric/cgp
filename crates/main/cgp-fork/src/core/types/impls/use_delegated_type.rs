use core::marker::PhantomData;

use cgp_fork_macro::cgp_provider;

use crate::core::component::{DelegateComponent, WithProvider};
use crate::core::types::TypeProviderComponent;
use crate::core::types::traits::TypeProvider;

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Components, Type> TypeProvider<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
