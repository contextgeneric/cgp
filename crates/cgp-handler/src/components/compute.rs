use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Computer)]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _tag: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Components, Delegate> Computer<Context, Code, Input>
    for UseDelegate<Components>
where
    Components: DelegateComponent<Code, Delegate = Delegate>,
    Delegate: Computer<Context, Code, Input>,
{
    type Output = Delegate::Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Self::Output {
        Delegate::compute(context, tag, input)
    }
}
