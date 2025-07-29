use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchMatchers;

pub struct MatchWithHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<'a, Context, Code, Input, Output, Remainder, Handlers> Computer<Context, Code, &'a Input>
    for MatchWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, Input::ExtractorRef<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &'a Input) -> Output {
        DispatchMatchers::compute(context, code, input.extractor_ref()).finalize_extract_result()
    }
}

#[cgp_provider]
impl<'a, Context, Code, Input, Output, Remainder, Handlers> TryComputer<Context, Code, &'a Input>
    for MatchWithHandlersRef<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>:
        TryComputer<Context, Code, Input::ExtractorRef<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: &'a Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::try_compute(context, code, input.extractor_ref())?
                .finalize_extract_result(),
        )
    }
}

#[cgp_provider]
impl<'a, Context, Code: Send, Input, Output, Remainder, Handlers> Handler<Context, Code, &'a Input>
    for MatchWithHandlersRef<Handlers>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasExtractorRef,
    DispatchMatchers<Handlers>:
        Handler<Context, Code, Input::ExtractorRef<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: &'a Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::handle(context, code, input.extractor_ref())
                .await?
                .finalize_extract_result(),
        )
    }
}
