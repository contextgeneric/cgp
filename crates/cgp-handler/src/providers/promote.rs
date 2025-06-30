use cgp_core::prelude::*;

use crate::{
    Computer, ComputerComponent, Handler, HandlerComponent, Producer, TryComputer,
    TryComputerComponent,
};

pub struct Promote<Provider>(pub PhantomData<Provider>);

pub struct TryPromote<Provider>(pub PhantomData<Provider>);

pub type Promote2<Provider> = Promote<Promote<Provider>>;

pub type Promote3<Provider> = Promote<Promote2<Provider>>;

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input> for Promote<Provider>
where
    Context: HasAsyncErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
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
        Provider::try_compute(context, tag, input)
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
impl<Context, Code, Input, Output, Error, Provider> Handler<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: CanRaiseAsyncError<Error>,
    Provider: Computer<Context, Code, Input, Output = Result<Output, Error>>,
    Code: Send,
    Input: Send,
    Output: Send,
    Error: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::compute(context, tag, input).map_err(Context::raise_error)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Error, Provider> TryComputer<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: CanRaiseError<Error>,
    Provider: Computer<Context, Code, Input, Output = Result<Output, Error>>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::compute(context, tag, input).map_err(Context::raise_error)
    }
}
