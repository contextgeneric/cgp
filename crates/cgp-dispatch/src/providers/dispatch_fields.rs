use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, HandleFieldValue, Handler, HandlerComponent, TryComputer,
    TryComputerComponent,
};

use crate::{DispatchHandlers, DispatchHandlersRef, ExtractFieldAndHandle};

pub struct DispatchFields<Provider = UseContext>(pub PhantomData<Provider>);

pub struct DispatchFieldsRef<Provider = UseContext>(pub PhantomData<Provider>);

pub type DispatchFieldValues = DispatchFields<HandleFieldValue<UseContext>>;

pub type DispatchFieldValueRefs = DispatchFieldsRef<HandleFieldValue<UseContext>>;

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
impl<Context, Code, Input, Output, Fields, Provider> TryComputer<Context, Code, Input>
    for DispatchFields<Provider>
where
    Context: HasErrorType,
    Input: HasFields<Fields = Fields>,
    Fields: FieldsToExtractFieldHandlers<Provider>,
    DispatchHandlers<Fields::Handlers>: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        DispatchHandlers::try_compute(context, code, input)
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
impl<Context, Code, Input, Output, Provider> Computer<Context, Code, &Input>
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

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputer<Context, Code, &Input>
    for DispatchFieldsRef<Provider>
where
    Context: HasErrorType,
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> DispatchHandlersRef<<Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers>:
        TryComputer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        DispatchHandlersRef::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input, Output: Send, Provider> Handler<Context, Code, &Input>
    for DispatchFieldsRef<Provider>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> DispatchHandlersRef<<Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers>:
        Handler<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        DispatchHandlersRef::handle(context, code, input).await
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
