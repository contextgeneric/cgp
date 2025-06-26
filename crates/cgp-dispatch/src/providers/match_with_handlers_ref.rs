use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchMatchers;

pub struct MatchWithHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, &Input>
    for MatchWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>: for<'b> Computer<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output: FinalizeExtractResult<Output = Output>,
    >,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        DispatchMatchers::compute(context, code, input.extractor_ref()).finalize_extract_result()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> TryComputer<Context, Code, &Input>
    for MatchWithHandlersRef<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>: for<'b> TryComputer<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output: FinalizeExtractResult<Output = Output>,
    >,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::try_compute(context, code, input.extractor_ref())?
                .finalize_extract_result(),
        )
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input, Output, Handlers> Handler<Context, Code, &Input>
    for MatchWithHandlersRef<Handlers>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasExtractorRef,
    DispatchMatchers<Handlers>: for<'b> Handler<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output: FinalizeExtractResult<Output = Output>,
    >,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::handle(_context, code, input.extractor_ref())
                .await?
                .finalize_extract_result(),
        )
    }
}
