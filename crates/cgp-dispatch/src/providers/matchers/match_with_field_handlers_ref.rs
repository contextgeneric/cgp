use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerRef, ComputerRefComponent, HandleFieldValue, Handler, HandlerRef,
    HandlerRefComponent, PromoteRef, TryComputer, TryComputerRef, TryComputerRefComponent,
};

use crate::providers::matchers::to_field_handlers::ToFieldHandlers;
use crate::MatchWithHandlersRef;

pub type MatchWithFieldHandlersRef<Provider = UseContext> =
    MatchWithFieldHandlersRefImpl<PromoteRef<Provider>>;

pub type MatchWithValueHandlersRef<Provider = UseContext> =
    MatchWithFieldHandlersRefImpl<HandleFieldValue<PromoteRef<Provider>>>;

pub struct MatchWithFieldHandlersRefImpl<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> ComputerRef<Context, Code, Input>
    for MatchWithFieldHandlersRefImpl<Provider>
where
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: ToFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<<Input::FieldsRef<'b> as ToFieldHandlers<Provider>>::Handlers>:
        Computer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn compute_ref(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        MatchWithHandlersRef::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputerRef<Context, Code, Input>
    for MatchWithFieldHandlersRefImpl<Provider>
where
    Context: HasErrorType,
    Input: HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: ToFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<<Input::FieldsRef<'b> as ToFieldHandlers<Provider>>::Handlers>:
        TryComputer<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    fn try_compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlersRef::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input, Output: Send, Provider> HandlerRef<Context, Code, Input>
    for MatchWithFieldHandlersRefImpl<Provider>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasFieldsRef,
    for<'b> Input::FieldsRef<'b>: ToFieldHandlers<Provider>,
    for<'b> MatchWithHandlersRef<<Input::FieldsRef<'b> as ToFieldHandlers<Provider>>::Handlers>:
        Handler<Context, Code, &'b Input, Output = Output>,
{
    type Output = Output;

    async fn handle_ref(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        MatchWithHandlersRef::handle(context, code, input).await
    }
}
