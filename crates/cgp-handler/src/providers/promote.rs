use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::{Computer, Handler, HandlerComponent};

#[cgp_new_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input> for Promote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Computer<Context, Code, Input, Output = Output>,
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
        Ok(Provider::compute(context, tag, input))
    }
}
