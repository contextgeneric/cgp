use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

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
impl<Context, Code, Tag, Input, Args, Output, Provider>
    TryComputer<Context, Code, (Field<Tag, Input>, Args)> for HandleFirstFieldValue<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, (Input, Args), Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Field<Tag, Input>, Args),
    ) -> Result<Self::Output, Context::Error> {
        Provider::try_compute(context, tag, (input.value, args))
    }
}

#[cgp_provider]
impl<Context, Code: Send, Tag: Send, Input: Send, Args: Send, Provider>
    Handler<Context, Code, (Field<Tag, Input>, Args)> for HandleFirstFieldValue<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Handler<Context, Code, (Input, Args)>,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Field<Tag, Input>, Args),
    ) -> Result<Self::Output, Context::Error> {
        Provider::handle(context, tag, (input.value, args)).await
    }
}
