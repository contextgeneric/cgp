use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, HandleFieldValue, Handler, HandlerComponent, TryComputer,
    TryComputerComponent,
};

use crate::providers::matchers::to_field_handlers::ToFieldHandlers;
use crate::MatchWithHandlers;

pub struct MatchWithFieldHandlers<Provider = UseContext>(pub PhantomData<Provider>);

pub type MatchWithValueHandlers<Provider = UseContext> =
    MatchWithFieldHandlers<HandleFieldValue<Provider>>;

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Provider> Computer<Context, Code, Input>
    for MatchWithFieldHandlers<Provider>
where
    Input: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<Provider>,
    MatchWithHandlers<Fields::Handlers>: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        MatchWithHandlers::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Provider> TryComputer<Context, Code, Input>
    for MatchWithFieldHandlers<Provider>
where
    Context: HasErrorType,
    Input: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<Provider>,
    MatchWithHandlers<Fields::Handlers>: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlers::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Fields, Provider> Handler<Context, Code, Input>
    for MatchWithFieldHandlers<Provider>
where
    Context: HasAsyncErrorType,
    Input: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<Provider>,
    MatchWithHandlers<Fields::Handlers>: Handler<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlers::handle(context, code, input).await
    }
}
