use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

pub struct HandleFirstFieldValue<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Tag, Input, Args, Output, Provider>
    Computer<Context, Code, (Field<Tag, Input>, Args)> for HandleFirstFieldValue<Provider>
where
    Provider: Computer<Context, Code, (Input, Args), Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Field<Tag, Input>, Args),
    ) -> Self::Output {
        Provider::compute(context, tag, (input.value, args))
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Input, Args, Provider>
    AsyncComputer<Context, Code, (Field<Tag, Input>, Args)> for HandleFirstFieldValue<Provider>
where
    Provider: AsyncComputer<Context, Code, (Input, Args)>,
{
    type Output = Provider::Output;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Field<Tag, Input>, Args),
    ) -> Self::Output {
        Provider::compute_async(context, tag, (input.value, args)).await
    }
}
