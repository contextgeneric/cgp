use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::{Compose, ContainsValue, MonadicBind, MonadicLift};

pub struct OkMonadic;

pub struct OkMonadicTrans<M>(pub PhantomData<M>);

impl<M, Provider> MonadicBind<Provider> for OkMonadicTrans<M> {
    type Provider = BindOk<M, Provider>;
}

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for OkMonadic {
    type Provider = ComposeOk<ProviderA, ProviderB>;
}

pub struct ComposeOk<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

pub struct BindOk<M, Cont>(pub PhantomData<(M, Cont)>);

#[cgp_provider]
impl<Context, Code, T, E1, E2, M, Cont> Computer<Context, Code, Result<T, E1>> for BindOk<M, Cont>
where
    Cont: Computer<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value = Result<T, E2>>
        + MonadicLift<Result<T, E2>, Cont::Output>,
{
    type Output = M::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T, E1>) -> Self::Output {
        match input {
            Err(value) => M::lift_output(Cont::compute(context, code, value)),
            Ok(err) => M::lift_value(Ok(err)),
        }
    }
}

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
