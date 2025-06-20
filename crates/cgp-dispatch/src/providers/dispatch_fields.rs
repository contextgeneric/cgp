use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

use crate::{DispatchHandlers, ExtractFieldAndHandle};

pub struct DispatchFields<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Provider> Computer<Context, Code, Input>
    for DispatchFields<Provider>
where
    Input: HasFields<Fields = Fields>,
    Fields: FieldsToExtractFieldHandlers<Provider>,
    DispatchHandlers<Fields::Handlers>: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        DispatchHandlers::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Fields, Provider> Handler<Context, Code, Input>
    for DispatchFields<Provider>
where
    Context: HasAsyncErrorType,
    Input: HasFields<Fields = Fields>,
    Fields: FieldsToExtractFieldHandlers<Provider>,
    DispatchHandlers<Fields::Handlers>: Handler<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        DispatchHandlers::handle(context, code, input).await
    }
}

trait FieldsToExtractFieldHandlers<Provider> {
    type Handlers;
}

impl<Tag, Value, RestFields, Provider> FieldsToExtractFieldHandlers<Provider>
    for Either<Field<Tag, Value>, RestFields>
where
    RestFields: FieldsToExtractFieldHandlers<Provider>,
{
    type Handlers = Cons<ExtractFieldAndHandle<Tag, Provider>, RestFields::Handlers>;
}

impl<Provider> FieldsToExtractFieldHandlers<Provider> for Void {
    type Handlers = Nil;
}
