use core::marker::PhantomData;

use cgp_component::WithProvider;

use crate::traits::has_type::ProvideType;

pub struct UseType<Type>(pub PhantomData<Type>);

pub type WithType<Type> = WithProvider<UseType<Type>>;

impl<Context, Tag, Type> ProvideType<Context, Tag> for UseType<Type> {
    type Type = Type;
}
