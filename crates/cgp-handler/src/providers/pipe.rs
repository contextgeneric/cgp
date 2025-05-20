use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

pub struct PipeHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Tag, Input, Output, CurrentHandler, RestHandlers> Handler<Context, Tag, Input>
    for PipeHandlers<Cons<CurrentHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Tag, Input>,
    PipeHandlers<RestHandlers>: Handler<Context, Tag, CurrentHandler::Output, Output = Output>,
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
        <PipeHandlers<RestHandlers>>::handle(context, tag, intermediate.into()).await
    }
}
