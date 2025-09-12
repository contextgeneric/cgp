use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

use crate::UseInputDelegate;

#[async_trait]
#[cgp_component {
    provider: AsyncComputer,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanComputeAsync<Code, Input> {
    type Output;

    async fn compute_async(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[async_trait]
#[cgp_component {
    provider: AsyncComputerRef,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanComputeAsyncRef<Code, Input> {
    type Output;

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
