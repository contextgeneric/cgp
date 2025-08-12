use cgp_core::prelude::*;

use crate::{
    AsyncComputer, AsyncComputerComponent, Computer, Handler, HandlerComponent, TryComputer,
};

pub struct PromoteAsync<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> AsyncComputer<Context, Code, Input>
    for PromoteAsync<Provider>
where
    Context: Async,
    Provider: Computer<Context, Code, Input, Output = Output>,
    Code: Send,
    Input: Send,
    Output: Send,
{
    type Output = Output;

    async fn compute_async(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        Provider::compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input>
    for PromoteAsync<Provider>
where
    Context: HasAsyncErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
    Code: Send,
    Input: Send,
    Output: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}
