use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::monadic::ident::IdentMonadic;
use crate::traits::{Compose, ContainsValue, MonadicBind, MonadicLift};

pub struct ErrMonadic;

pub struct ErrMonadicTrans<M>(pub PhantomData<M>);

impl<M, Provider> MonadicBind<Provider> for ErrMonadicTrans<M> {
    type Provider = BindErr<M, Provider>;
}

impl<Provider> MonadicBind<Provider> for ErrMonadic {
    type Provider = BindErr<IdentMonadic, Provider>;
}

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for ErrMonadic {
    type Provider = ComposeErr<ProviderA, ProviderB>;
}

pub struct ComposeErr<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

pub struct BindErr<M, Cont>(pub PhantomData<(M, Cont)>);

#[cgp_provider]
impl<Context, Code, T1, T2, E, M, Cont> Computer<Context, Code, Result<T1, E>> for BindErr<M, Cont>
where
    Cont: Computer<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>>
        + MonadicLift<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T1, E>) -> Self::Output {
        match input {
            Ok(value) => M::lift_output(Cont::compute(context, code, value)),
            Err(err) => M::lift_value(Err(err)),
        }
    }
}

#[cgp_provider]
impl<Context, Code, T1, T2, E, M, Cont> TryComputer<Context, Code, Result<T1, E>> for BindErr<M, Cont>
where
    Context: HasErrorType,
    Cont: TryComputer<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>>
        + MonadicLift<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    fn try_compute(context: &Context, code: PhantomData<Code>, input: Result<T1, E>) -> Result<Self::Output, Context::Error> {
        match input {
            Ok(value) => Ok(M::lift_output(Cont::try_compute(context, code, value)?)),
            Err(err) => Ok(M::lift_value(Err(err))),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, T1: Send, T2, E: Send, M, Cont> Handler<Context, Code, Result<T1, E>> for BindErr<M, Cont>
where
    Context: HasAsyncErrorType,
    Cont: Handler<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>>
        + MonadicLift<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    async fn handle(context: &Context, code: PhantomData<Code>, input: Result<T1, E>) -> Result<Self::Output, Context::Error> {
        match input {
            Ok(value) => Ok(M::lift_output(Cont::handle(context, code, value).await?)),
            Err(err) => Ok(M::lift_value(Err(err))),
        }
    }
}

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
