use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

/// The handler-family component for a computation that must await but cannot fail: its async method
/// returns the Output directly rather than a Result.
///
/// See
/// [`CanComputeAsync`](https://contextgeneric.dev/docs/reference/components/handler/async_computer)
/// .
#[async_trait]
#[cgp_component(AsyncComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanComputeAsync<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the input.
    async fn compute_async(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

/// The handler-family component for a computation that awaits, cannot fail, and only reads its
/// input, which its async method takes as &Input.
///
/// See
/// [`CanComputeAsyncRef`](https://contextgeneric.dev/docs/reference/components/handler/async_computer_ref)
/// .
#[async_trait]
#[cgp_component(AsyncComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanComputeAsyncRef<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the borrowed input.
    async fn compute_async_ref(&self, _code: PhantomData<Code>, input: &Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> AsyncComputer<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanComputeAsync<Code, Input, Output = Output>,
{
    type Output = Output;

    async fn compute_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context
            .get_field(PhantomData)
            .compute_async(code, input)
            .await
    }
}
