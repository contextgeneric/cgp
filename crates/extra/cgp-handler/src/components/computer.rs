use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

/// The handler-family component for a synchronous computation that cannot fail: a provider turns an
/// Input into an Output under a Code tag.
///
/// See [`CanCompute`](https://contextgeneric.dev/docs/reference/components/handler/computer).
#[cgp_component(Computer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanCompute<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the input.
    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

/// The handler-family component for a synchronous computation that cannot fail and only reads its
/// input, which it takes as &Input.
///
/// See [`CanComputeRef`](https://contextgeneric.dev/docs/reference/components/handler/computer_ref)
/// .
#[cgp_component(ComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanComputeRef<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the borrowed input.
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
