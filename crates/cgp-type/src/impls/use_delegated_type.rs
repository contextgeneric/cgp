use core::marker::PhantomData;

use cgp_component::{DelegateComponent, IsProviderFor, WithProvider};
use cgp_macro::cgp_provider;

use crate::TypeComponent;
use crate::traits::ProvideType;

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;

#[cgp_provider(TypeComponent)]
impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
