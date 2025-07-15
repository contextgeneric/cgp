use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::Compose;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, M, Providers> Computer<Context, Code, Input>
    for PipeMonadic<M, Providers>
where
    Providers: PipeComputer<M, Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output {
        Providers::compute(context, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, M, Providers> TryComputer<Context, Code, Input>
    for PipeMonadic<M, Providers>
where
    Context: HasErrorType,
    Providers: PipeTryComputer<M, Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Providers::try_compute(context, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output, M, Providers> Handler<Context, Code, Input>
    for PipeMonadic<M, Providers>
where
    Context: HasAsyncErrorType,
    Providers: PipeHandler<M, Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Providers::handle(context, input).await
    }
}

trait PipeComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, input: Input) -> Self::Output;
}

impl<Context, Code, Input, M, ProviderA, ProviderB, RestProviders, OutProvider>
    PipeComputer<M, Context, Code, Input> for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    M: Compose<ProviderA, PipeMonadic<M, Cons<ProviderB, RestProviders>>, Provider = OutProvider>,
    OutProvider: Computer<Context, Code, Input>,
{
    type Output = OutProvider::Output;

    fn compute(context: &Context, input: Input) -> Self::Output {
        OutProvider::compute(context, PhantomData, input)
    }
}

impl<Context, Code, Input, M, Provider> PipeComputer<M, Context, Code, Input>
    for Cons<Provider, Nil>
where
    Provider: Computer<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn compute(context: &Context, input: Input) -> Self::Output {
        Provider::compute(context, PhantomData, input)
    }
}

trait PipeTryComputer<M, Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    fn try_compute(context: &Context, input: Input) -> Result<Self::Output, Context::Error>;
}

impl<Context, Code, Input, M, ProviderA, ProviderB, RestProviders, OutProvider>
    PipeTryComputer<M, Context, Code, Input> for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Context: HasErrorType,
    M: Compose<ProviderA, PipeMonadic<M, Cons<ProviderB, RestProviders>>, Provider = OutProvider>,
    OutProvider: TryComputer<Context, Code, Input>,
{
    type Output = OutProvider::Output;

    fn try_compute(context: &Context, input: Input) -> Result<Self::Output, Context::Error> {
        OutProvider::try_compute(context, PhantomData, input)
    }
}

impl<Context, Code, Input, M, Provider> PipeTryComputer<M, Context, Code, Input>
    for Cons<Provider, Nil>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn try_compute(
        context: &Context,
        input: Input,
    ) -> Result<Self::Output, <Context as HasErrorType>::Error> {
        Provider::try_compute(context, PhantomData, input)
    }
}

#[async_trait]
trait PipeHandler<M, Context, Code, Input>
where
    Context: HasAsyncErrorType,
{
    type Output;

    async fn handle(context: &Context, input: Input) -> Result<Self::Output, Context::Error>;
}

impl<Context, Code: Send, Input: Send, M, ProviderA, ProviderB, RestProviders, OutProvider>
    PipeHandler<M, Context, Code, Input> for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Context: HasAsyncErrorType,
    M: Compose<ProviderA, PipeMonadic<M, Cons<ProviderB, RestProviders>>, Provider = OutProvider>,
    OutProvider: Handler<Context, Code, Input>,
{
    type Output = OutProvider::Output;

    async fn handle(context: &Context, input: Input) -> Result<Self::Output, Context::Error> {
        OutProvider::handle(context, PhantomData, input).await
    }
}

impl<Context, Code: Send, Input: Send, M, Provider> PipeHandler<M, Context, Code, Input>
    for Cons<Provider, Nil>
where
    Context: HasAsyncErrorType,
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        input: Input,
    ) -> Result<Self::Output, <Context as HasErrorType>::Error> {
        Provider::handle(context, PhantomData, input).await
    }
}
