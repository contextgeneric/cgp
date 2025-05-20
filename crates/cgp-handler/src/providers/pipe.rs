use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct PipeHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Tag, CurrentHandler, RestHandlers> Handler<Context, Tag>
    for PipeHandlers<Cons<CurrentHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Tag>,
    PipeHandlers<RestHandlers>: Handler<Context, Tag>,
    Tag: Send,
    <PipeHandlers<RestHandlers> as Handler<Context, Tag>>::Input: From<CurrentHandler::Output>,
{
    type Input = CurrentHandler::Input;

    type Output = <PipeHandlers<RestHandlers> as Handler<Context, Tag>>::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Self::Input,
    ) -> Result<Self::Output, Context::Error> {
        let intermediate = CurrentHandler::handle(context, tag, input).await?;
        <PipeHandlers<RestHandlers>>::handle(context, tag, intermediate.into()).await
    }
}

#[cgp_new_provider]
impl<Context, Tag, HandlerA, HandlerB> Handler<Context, Tag> for PipeTwoHandlers<HandlerA, HandlerB>
where
    Context: HasAsyncErrorType,
    HandlerA: Handler<Context, Tag>,
    HandlerB: Handler<Context, Tag>,
    HandlerB::Input: From<HandlerA::Output>,
    Tag: Send,
{
    type Input = HandlerA::Input;

    type Output = HandlerB::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Self::Input,
    ) -> Result<Self::Output, Context::Error> {
        let intermediate = HandlerA::handle(context, tag, input).await?;
        HandlerB::handle(context, tag, intermediate.into()).await
    }
}
