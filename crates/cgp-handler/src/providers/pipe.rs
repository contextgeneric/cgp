use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::components::*;

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
