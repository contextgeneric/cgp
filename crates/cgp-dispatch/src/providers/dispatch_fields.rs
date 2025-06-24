use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

use crate::{DispatchHandlers, DispatchHandlersRef, ExtractFieldAndHandle};

pub struct DispatchFields<Provider = UseContext>(pub PhantomData<Provider>);

pub struct DispatchFieldsRef<Provider = UseContext>(pub PhantomData<Provider>);

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

#[cgp_provider]
impl<'a, Context, Code, Input, Output, Provider> Computer<Context, Code, &'a Input>
    for DispatchFieldsRef<Provider>
where
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> DispatchHandlersRef<<Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers>:
        Computer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        DispatchHandlersRef::compute(context, code, input)
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
