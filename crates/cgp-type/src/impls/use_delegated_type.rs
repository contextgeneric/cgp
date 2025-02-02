use core::marker::PhantomData;

use cgp_component::{DelegateComponent, IsProviderFor, WithProvider};
use cgp_component_macro::cgp_provider;

use crate::traits::ProvideType;
use crate::TypeComponent;

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeComponent)]
impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
