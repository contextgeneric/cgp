use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct Pipe<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Tag, Input, Output, CurrentHandler, RestHandlers> Handler<Context, Tag, Input>
    for Pipe<Cons<CurrentHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Tag, Input>,
    Pipe<RestHandlers>: Handler<Context, Tag, CurrentHandler::Output, Output = Output>,
    Tag: Send,
    Input: Send,
    Output: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let intermediate = CurrentHandler::handle(context, tag, input).await?;
        <Pipe<RestHandlers>>::handle(context, tag, intermediate.into()).await
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
