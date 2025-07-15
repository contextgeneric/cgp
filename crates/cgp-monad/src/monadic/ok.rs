use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::Compose;

pub struct OkMonadic;

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for OkMonadic {
    type Provider = ComposeOk<ProviderA, ProviderB>;
}

pub struct ComposeOk<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB, T, E1, E2> Computer<Context, Code, Input>
    for ComposeOk<ProviderA, ProviderB>
where
    ProviderA: Computer<Context, Code, Input, Output = Result<T, E1>>,
    ProviderB: Computer<Context, Code, E1, Output = Result<T, E2>>,
{
    type Output = Result<T, E2>;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let res = ProviderA::compute(context, code, input);
        match res {
            Err(value) => ProviderB::compute(context, code, value),
            Ok(err) => Ok(err),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB, T, E1, E2> TryComputer<Context, Code, Input>
    for ComposeOk<ProviderA, ProviderB>
where
    Context: HasErrorType,
    ProviderA: TryComputer<Context, Code, Input, Output = Result<T, E1>>,
    ProviderB: TryComputer<Context, Code, E1, Output = Result<T, E2>>,
{
    type Output = Result<T, E2>;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::try_compute(context, code, input)?;
        match res {
            Err(value) => ProviderB::try_compute(context, code, value),
            Ok(err) => Ok(Ok(err)),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, ProviderA, ProviderB, T: Send, E1: Send, E2: Send>
    Handler<Context, Code, Input> for ComposeOk<ProviderA, ProviderB>
where
    Context: HasAsyncErrorType,
    ProviderA: Handler<Context, Code, Input, Output = Result<T, E1>>,
    ProviderB: Handler<Context, Code, E1, Output = Result<T, E2>>,
{
    type Output = Result<T, E2>;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::handle(context, code, input).await?;
        match res {
            Err(value) => ProviderB::handle(context, code, value).await,
            Ok(err) => Ok(Ok(err)),
        }
    }
}
