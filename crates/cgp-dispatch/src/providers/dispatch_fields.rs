use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, HandleFieldValue, Handler, HandlerComponent, TryComputer,
    TryComputerComponent,
};

use crate::{ExtractFieldAndHandle, MatchWithHandlers, MatchWithHandlersRef};

pub struct MatchWithFieldHandlers<Provider = UseContext>(pub PhantomData<Provider>);

pub struct MatchWithFieldHandlersRef<Provider = UseContext>(pub PhantomData<Provider>);

pub type MatchWithValueHandlers = MatchWithFieldHandlers<HandleFieldValue<UseContext>>;

pub type MatchWithValueHandlersRef = MatchWithFieldHandlersRef<HandleFieldValue<UseContext>>;

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Provider> Computer<Context, Code, Input>
    for MatchWithFieldHandlers<Provider>
where
    Input: HasFields<Fields = Fields>,
    Fields: FieldsToExtractFieldHandlers<Provider>,
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
    Fields: FieldsToExtractFieldHandlers<Provider>,
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
    Fields: FieldsToExtractFieldHandlers<Provider>,
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

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Computer<Context, Code, &Input>
    for MatchWithFieldHandlersRef<Provider>
where
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<
        <Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers,
    >: Computer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        MatchWithHandlersRef::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputer<Context, Code, &Input>
    for MatchWithFieldHandlersRef<Provider>
where
    Context: HasErrorType,
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<
        <Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers,
    >: TryComputer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlersRef::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input, Output: Send, Provider> Handler<Context, Code, &Input>
    for MatchWithFieldHandlersRef<Provider>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: FieldsToExtractFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<
        <Input::FieldsRef<'b> as FieldsToExtractFieldHandlers<Provider>>::Handlers,
    >: Handler<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlersRef::handle(context, code, input).await
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
