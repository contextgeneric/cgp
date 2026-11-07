use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

#[async_trait]
#[cgp_component(AsyncComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// An asynchronous, infallible transform.
///
/// See [`CanComputeAsync`](https://contextgeneric.dev/docs/reference/components/handler/async_computer).
pub trait CanComputeAsync<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the input.
    async fn compute_async(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[async_trait]
#[cgp_component(AsyncComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// An asynchronous, infallible transform that borrows its input.
///
/// See [`CanComputeAsyncRef`](https://contextgeneric.dev/docs/reference/components/handler/async_computer_ref).
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
