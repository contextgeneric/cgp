use core::marker::PhantomData;

use cgp_component::DelegateComponent;

use crate::traits::ProvideType;

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
