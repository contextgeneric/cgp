use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Handler)]
#[async_trait]
pub trait CanHandle<Code: Send, Input: Send>: HasAsyncErrorType {
    type Output: Send;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

#[cgp_provider]
impl<Context, Code, Input, Components, Delegate> Handler<Context, Code, Input>
    for UseDelegate<Components>
where
    Context: HasAsyncErrorType,
    Components: DelegateComponent<Code, Delegate = Delegate>,
    Delegate: Handler<Context, Code, Input>,
    Code: Send,
    Input: Send,
{
    type Output = Delegate::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Delegate::handle(context, tag, input).await
    }
}
