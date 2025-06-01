use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct PipeHandlers<Providers>(pub PhantomData<Providers>);

#[cgp_provider]
impl<Context, Code, Input, Output, CurrentProvider, RestProviders> Handler<Context, Code, Input>
    for PipeHandlers<Cons<CurrentProvider, RestProviders>>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Handler<Context, Code, Input>,
    PipeHandlers<RestProviders>: Handler<Context, Code, CurrentProvider::Output, Output = Output>,
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
        <PipeHandlers<RestProviders>>::handle(context, tag, intermediate).await
    }
}

#[cgp_provider]
impl<Context, Tag, Input> Handler<Context, Tag, Input> for PipeHandlers<Nil>
where
    Context: HasAsyncErrorType,
    Tag: Send,
    Input: Send,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Input, Context::Error> {
        Ok(input)
    }
}

#[cgp_provider]
impl<Context, Tag, Input, Output, CurrentProvider, RestProviders> Computer<Context, Tag, Input>
    for PipeHandlers<Cons<CurrentProvider, RestProviders>>
where
    CurrentProvider: Computer<Context, Tag, Input>,
    PipeHandlers<RestProviders>: Computer<Context, Tag, CurrentProvider::Output, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        <PipeHandlers<RestProviders>>::compute(context, tag, intermediate)
    }
}

#[cgp_provider]
impl<Context, Tag, Input> Computer<Context, Tag, Input> for PipeHandlers<Nil> {
    type Output = Input;

    fn compute(_context: &Context, _tag: PhantomData<Tag>, input: Input) -> Input {
        input
    }
}
