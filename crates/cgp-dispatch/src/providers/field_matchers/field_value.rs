use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

pub struct HandleFieldValue<Provider = UseContext>(pub PhantomData<Provider>);

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
impl<Context: Async, Code: Send, Tag: Send, Input: Send, Provider>
    AsyncComputer<Context, Code, Field<Tag, Input>> for HandleFieldValue<Provider>
where
    Provider: AsyncComputer<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        input: Field<Tag, Input>,
    ) -> Self::Output {
        Provider::compute_async(context, tag, input.value).await
    }
}
