use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::{Computer, ComputerComponent, Handler, HandlerComponent, Producer};

pub struct Promote<Provider>(pub PhantomData<Provider>);

pub type Promote2<Provider> = Promote<Promote<Provider>>;

pub type Promote3<Provider> = Promote<Promote2<Provider>>;

#[cgp_provider]
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

#[cgp_new_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input> for TryPromote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Computer<Context, Code, Input, Output = Result<Output, Context::Error>>,
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
        Provider::compute(context, tag, input)
    }
}
