use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Computer)]
pub trait CanCompute<Tag, Input> {
    type Output;

    fn compute(&self, _tag: PhantomData<Tag>, input: Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Tag, Input, Components, Delegate> Computer<Context, Tag, Input>
    for UseDelegate<Components>
where
    Components: DelegateComponent<Tag, Delegate = Delegate>,
    Delegate: Computer<Context, Tag, Input>,
{
    type Output = Delegate::Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Self::Output {
        Delegate::compute(context, tag, input)
    }
}
