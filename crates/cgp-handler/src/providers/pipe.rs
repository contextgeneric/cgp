use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct PipeHandlers<Providers>(pub PhantomData<Providers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Providers> Handler<Context, Code, Input>
    for PipeHandlers<Providers>
where
    Context: HasAsyncErrorType,
    Providers: PipeHandler<Context, Code, Input, Output = Output>,
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
        Providers::handle(context, tag, input).await
    }
}

#[cgp_provider]
impl<Context, Tag, Input, Output, Providers> TryComputer<Context, Tag, Input>
    for PipeHandlers<Providers>
where
    Context: HasErrorType,
    Providers: PipeTryComputer<Context, Tag, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Providers::try_compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Tag, Input, Output, Providers> Computer<Context, Tag, Input>
    for PipeHandlers<Providers>
where
    Providers: PipeComputer<Context, Tag, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Output {
        Providers::compute(context, tag, input)
    }
}

#[async_trait]
trait PipeHandler<Context, Code, Input>
where
    Context: HasAsyncErrorType,
{
    type Output;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error>;
}

impl<Context, Code, Input, Output, CurrentProvider, RestProviders> PipeHandler<Context, Code, Input>
    for Cons<CurrentProvider, RestProviders>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Handler<Context, Code, Input>,
    RestProviders: PipeHandler<Context, Code, CurrentProvider::Output, Output = Output>,
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
        let intermediate = CurrentProvider::handle(context, tag, input).await?;
        RestProviders::handle(context, tag, intermediate).await
    }
}

impl<Context, Code, Input> PipeHandler<Context, Code, Input> for Nil
where
    Context: HasAsyncErrorType,
    Code: Send,
    Input: Send,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Input, Context::Error> {
        Ok(input)
    }
}

trait PipeTryComputer<Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    fn try_compute(
        context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error>;
}

impl<Context, Tag, Input, Output, CurrentProvider, RestProviders>
    PipeTryComputer<Context, Tag, Input> for Cons<CurrentProvider, RestProviders>
where
    Context: HasErrorType,
    CurrentProvider: TryComputer<Context, Tag, Input>,
    RestProviders: PipeTryComputer<Context, Tag, CurrentProvider::Output, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let intermediate = CurrentProvider::try_compute(context, tag, input)?;
        RestProviders::try_compute(context, tag, intermediate)
    }
}

impl<Context, Code, Input> PipeTryComputer<Context, Code, Input> for Nil
where
    Context: HasErrorType,
{
    type Output = Input;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Input, Context::Error> {
        Ok(input)
    }
}

trait PipeComputer<Context, Code, Input> {
    type Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

impl<Context, Tag, Input, Output, CurrentProvider, RestProviders> PipeComputer<Context, Tag, Input>
    for Cons<CurrentProvider, RestProviders>
where
    CurrentProvider: Computer<Context, Tag, Input>,
    RestProviders: PipeComputer<Context, Tag, CurrentProvider::Output, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        RestProviders::compute(context, tag, intermediate)
    }
}

impl<Context, Code, Input> PipeComputer<Context, Code, Input> for Nil {
    type Output = Input;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}
