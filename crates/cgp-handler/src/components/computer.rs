use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

use crate::UseInputDelegate;

#[cgp_component {
    provider: Computer,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_component {
    provider: ComputerRef,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanComputeRef<Code, Input> {
    type Output;

    fn compute_ref(&self, _code: PhantomData<Code>, input: &Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> Computer<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanCompute<Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context.get_field(PhantomData).compute(code, input)
    }
}
