use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct Pipe<Providers>(pub PhantomData<Providers>);

#[cgp_provider]
impl<Context, Tag, Input, Output, CurrentProvider, RestProviders> Handler<Context, Tag, Input>
    for Pipe<Cons<CurrentProvider, RestProviders>>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Handler<Context, Tag, Input>,
    Pipe<RestProviders>: Handler<Context, Tag, CurrentProvider::Output, Output = Output>,
    Tag: Send,
    Input: Send,
    Output: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let intermediate = CurrentProvider::handle(context, tag, input).await?;
        <Pipe<RestProviders>>::handle(context, tag, intermediate.into()).await
    }
}

#[cgp_provider]
impl<Context, Tag, Input> Handler<Context, Tag, Input> for Pipe<Nil>
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
    for Pipe<Cons<CurrentProvider, RestProviders>>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Computer<Context, Tag, Input>,
    Pipe<RestProviders>: Computer<Context, Tag, CurrentProvider::Output, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        <Pipe<RestProviders>>::compute(context, tag, intermediate.into())
    }
}

#[cgp_provider]
impl<Context, Tag, Input> Computer<Context, Tag, Input> for Pipe<Nil> {
    type Output = Input;

    fn compute(_context: &Context, _tag: PhantomData<Tag>, input: Input) -> Input {
        input
    }
}
