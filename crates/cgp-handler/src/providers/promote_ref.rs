use core::ops::Deref;

use cgp_core::prelude::*;

use crate::{
    Computer, ComputerComponent, ComputerRef, ComputerRefComponent, Handler, HandlerComponent,
    HandlerRef, HandlerRefComponent, TryComputer, TryComputerComponent, TryComputerRef,
    TryComputerRefComponent,
};

pub struct PromoteRef<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Target, Provider> Handler<Context, Code, Input> for PromoteRef<Provider>
where
    Context: HasAsyncErrorType,
    Provider: HandlerRef<Context, Code, Target>,
    Input: Deref<Target = Target> + Async,
    Code: Send,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Provider::handle_ref(context, tag, input.deref()).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> HandlerRef<Context, Code, Input>
    for PromoteRef<Provider>
where
    Context: HasAsyncErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Input, Output = Output>,
    Code: Send,
    Input: Sync,
{
    type Output = Output;

    async fn handle_ref(
        context: &Context,
        tag: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Context::Error> {
        Provider::handle(context, tag, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Target, Provider> TryComputer<Context, Code, Input>
    for PromoteRef<Provider>
where
    Context: HasErrorType,
    Provider: TryComputerRef<Context, Code, Target>,
    Input: Deref<Target = Target>,
{
    type Output = Provider::Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Provider::try_compute_ref(context, tag, input.deref())
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> TryComputerRef<Context, Code, Input>
    for PromoteRef<Provider>
where
    Context: HasErrorType,
    Provider: for<'a> TryComputer<Context, Code, &'a Input, Output = Output>,
{
    type Output = Output;

    fn try_compute_ref(
        context: &Context,
        tag: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Target, Provider> Computer<Context, Code, Input> for PromoteRef<Provider>
where
    Provider: ComputerRef<Context, Code, Target>,
    Input: Deref<Target = Target>,
{
    type Output = Provider::Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Self::Output {
        Provider::compute_ref(context, tag, input.deref())
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> ComputerRef<Context, Code, Input>
    for PromoteRef<Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Input, Output = Output>,
{
    type Output = Output;

    fn compute_ref(context: &Context, tag: PhantomData<Code>, input: &Input) -> Self::Output {
        Provider::compute(context, tag, input)
    }
}
