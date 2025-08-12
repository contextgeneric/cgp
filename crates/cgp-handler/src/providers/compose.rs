use cgp_core::prelude::*;

use crate::{
    AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent, Handler, HandlerComponent,
    TryComputer, TryComputerComponent,
};

pub struct ComposeHandlers<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB> Computer<Context, Code, Input>
    for ComposeHandlers<ProviderA, ProviderB>
where
    ProviderA: Computer<Context, Code, Input>,
    ProviderB: Computer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let intermediary = ProviderA::compute(context, code, input);
        ProviderB::compute(context, code, intermediary)
    }
}

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB> TryComputer<Context, Code, Input>
    for ComposeHandlers<ProviderA, ProviderB>
where
    Context: HasErrorType,
    ProviderA: TryComputer<Context, Code, Input>,
    ProviderB: TryComputer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let intermediary = ProviderA::try_compute(context, code, input)?;
        ProviderB::try_compute(context, code, intermediary)
    }
}

#[cgp_provider]
impl<Context: Async, Code: Send, Input: Send, ProviderA, ProviderB>
    AsyncComputer<Context, Code, Input> for ComposeHandlers<ProviderA, ProviderB>
where
    ProviderA: AsyncComputer<Context, Code, Input>,
    ProviderB: AsyncComputer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        let intermediary = ProviderA::compute_async(context, code, input).await;
        ProviderB::compute_async(context, code, intermediary).await
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, ProviderA, ProviderB> Handler<Context, Code, Input>
    for ComposeHandlers<ProviderA, ProviderB>
where
    Context: HasAsyncErrorType,
    ProviderA: Handler<Context, Code, Input>,
    ProviderB: Handler<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let intermediary = ProviderA::handle(context, code, input).await?;
        ProviderB::handle(context, code, intermediary).await
    }
}
