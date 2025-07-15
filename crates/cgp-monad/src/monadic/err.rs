use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::Compose;

pub struct ErrMonadic;

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for ErrMonadic {
    type Provider = ComposeErr<ProviderA, ProviderB>;
}

pub struct ComposeErr<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB, T1, T2, E> Computer<Context, Code, Input>
    for ComposeErr<ProviderA, ProviderB>
where
    ProviderA: Computer<Context, Code, Input, Output = Result<T1, E>>,
    ProviderB: Computer<Context, Code, T1, Output = Result<T2, E>>,
{
    type Output = Result<T2, E>;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let res = ProviderA::compute(context, code, input);
        match res {
            Ok(value) => ProviderB::compute(context, code, value),
            Err(err) => Err(err),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB, T1, T2, E> TryComputer<Context, Code, Input>
    for ComposeErr<ProviderA, ProviderB>
where
    Context: HasErrorType,
    ProviderA: TryComputer<Context, Code, Input, Output = Result<T1, E>>,
    ProviderB: TryComputer<Context, Code, T1, Output = Result<T2, E>>,
{
    type Output = Result<T2, E>;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::try_compute(context, code, input)?;
        match res {
            Ok(value) => ProviderB::try_compute(context, code, value),
            Err(err) => Ok(Err(err)),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, ProviderA, ProviderB, T1: Send, T2: Send, E: Send>
    Handler<Context, Code, Input> for ComposeErr<ProviderA, ProviderB>
where
    Context: HasAsyncErrorType,
    ProviderA: Handler<Context, Code, Input, Output = Result<T1, E>>,
    ProviderB: Handler<Context, Code, T1, Output = Result<T2, E>>,
{
    type Output = Result<T2, E>;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::handle(context, code, input).await?;
        match res {
            Ok(value) => ProviderB::handle(context, code, value).await,
            Err(err) => Ok(Err(err)),
        }
    }
}
