use core::marker::PhantomData;

use cgp_component::{IsProviderFor, WithProvider};
use cgp_macro::cgp_provider;

use crate::traits::ProvideType;
use crate::TypeComponent;

pub struct UseType<Type>(pub PhantomData<Type>);

pub type WithType<Type> = WithProvider<UseType<Type>>;

#[cgp_provider(TypeComponent)]
impl<Context, Tag, Type> ProvideType<Context, Tag> for UseType<Type> {
    type Type = Type;
}
