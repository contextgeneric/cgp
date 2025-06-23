use cgp_core::prelude::*;

use crate::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct HandleFieldValue<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Tag, Input, Output, Provider> Computer<Context, Code, Field<Tag, Input>>
    for HandleFieldValue<Provider>
where
    Provider: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Field<Tag, Input>,
    ) -> Self::Output {
        Provider::compute(context, tag, input.value)
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Input, Output, Provider> TryComputer<Context, Code, Field<Tag, Input>>
    for HandleFieldValue<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Field<Tag, Input>,
    ) -> Result<Self::Output, Context::Error> {
        Provider::try_compute(context, tag, input.value)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Tag: Send, Input: Send, Provider>
    Handler<Context, Code, Field<Tag, Input>> for HandleFieldValue<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Field<Tag, Input>,
    ) -> Result<Self::Output, Context::Error> {
        Provider::handle(context, tag, input.value).await
    }
}
