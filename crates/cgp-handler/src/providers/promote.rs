use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::{Computer, Handler, HandlerComponent};

#[cgp_new_provider]
impl<Context, Tag, Input, Output, Provider> Handler<Context, Tag, Input> for Promote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Computer<Context, Tag, Input, Output = Output>,
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
        Ok(Provider::compute(context, tag, input))
    }
}
