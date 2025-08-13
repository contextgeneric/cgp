use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchMatchers;

pub struct MatchWithHandlersMut<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<'a, Context, Code, Input, Output, Remainder, Handlers> Computer<Context, Code, &'a mut Input>
    for MatchWithHandlersMut<Handlers>
where
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, Input::ExtractorMut<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &'a mut Input) -> Output {
        DispatchMatchers::compute(context, code, input.extractor_mut()).finalize_extract_result()
    }
}

#[cgp_provider]
impl<'a, Context, Code, Input, Output, Remainder, Handlers>
    TryComputer<Context, Code, &'a mut Input> for MatchWithHandlersMut<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>:
        TryComputer<Context, Code, Input::ExtractorMut<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: &'a mut Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::try_compute(context, code, input.extractor_mut())?
                .finalize_extract_result(),
        )
    }
}

#[cgp_provider]
impl<'a, Context, Code: Send, Input, Output, Remainder, Handlers>
    Handler<Context, Code, &'a mut Input> for MatchWithHandlersMut<Handlers>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasExtractorMut,
    DispatchMatchers<Handlers>:
        Handler<Context, Code, Input::ExtractorMut<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: &'a mut Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::handle(context, code, input.extractor_mut())
                .await?
                .finalize_extract_result(),
        )
    }
}
