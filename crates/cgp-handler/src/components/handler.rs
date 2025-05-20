use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Handler)]
#[async_trait]
pub trait CanHandle<Tag: Send, Input: Send>: HasAsyncErrorType {
    type Output: Send;

    async fn handle(
        &self,
        _tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

#[cgp_provider]
impl<Context, Tag, Input, Components, Delegate> Handler<Context, Tag, Input>
    for UseDelegate<Components>
where
    Context: HasAsyncErrorType,
    Components: DelegateComponent<Tag, Delegate = Delegate>,
    Delegate: Handler<Context, Tag, Input>,
    Tag: Send,
    Input: Send,
{
    type Output = Delegate::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Delegate::handle(context, tag, input).await
    }
}
