use core::marker::PhantomData;

use crate::traits::has_type::ProvideType;
use crate::types::with_type::WithTypeProvider;

pub struct UseType<Type>(pub PhantomData<Type>);

pub type WithType<Type> = WithTypeProvider<UseType<Type>>;

impl<Context, Tag, Type> ProvideType<Context, Tag> for UseType<Type> {
    type Type = Type;
}
