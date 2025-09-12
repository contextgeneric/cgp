use cgp_core::prelude::*;

use crate::{
    AsyncComputer, Computer, ComputerComponent, Handler, HandlerComponent, Producer, TryComputer,
    TryComputerComponent,
};

pub struct Promote<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Computer<Context, Code, Input> for Promote<Provider>
where
    Provider: Producer<Context, Code, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        Provider::produce(context, code)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputer<Context, Code, Input> for Promote<Provider>
where
    Context: HasErrorType,
    Provider: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(Provider::compute(context, code, input))
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input> for Promote<Provider>
where
    Context: HasErrorType,
    Provider: AsyncComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(Provider::compute_async(context, code, input).await)
    }
}
