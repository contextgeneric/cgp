use cgp_core::prelude::*;

use crate::{
    AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent, Handler, HandlerComponent,
    TryComputer, TryComputerComponent,
};

pub struct TryPromote<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputer<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: HasErrorType,
    Provider: Computer<Context, Code, Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> Computer<Context, Code, Input> for TryPromote<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Result<Output, Context::Error>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output, Provider> Handler<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: AsyncComputer<Context, Code, Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::compute_async(context, tag, input).await
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Provider, Output> AsyncComputer<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Handler<Context, Code, Input, Output = Output>,
{
    type Output = Result<Output, Context::Error>;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::handle(context, tag, input).await
    }
}
