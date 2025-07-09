use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::dispatch_matchers2::{DispatchMatchers2, OnlyError};

pub struct MatchWithHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, &Input>
    for MatchWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers2<Handlers>: for<'b> Computer<
        Context,
        Code,
        OnlyError<Input::ExtractorRef<'b>>,
        Output: FinalizeExtractResult<Output = Output>,
    >,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        DispatchMatchers2::compute(context, code, OnlyError(input.extractor_ref()))
            .finalize_extract_result()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> TryComputer<Context, Code, &Input>
    for MatchWithHandlersRef<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorRef,
    DispatchMatchers2<Handlers>: for<'b> TryComputer<
        Context,
        Code,
        OnlyError<Input::ExtractorRef<'b>>,
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
            DispatchMatchers2::try_compute(context, code, OnlyError(input.extractor_ref()))?
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
    DispatchMatchers2<Handlers>: for<'b> Handler<
        Context,
        Code,
        OnlyError<Input::ExtractorRef<'b>>,
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
            DispatchMatchers2::handle(_context, code, OnlyError(input.extractor_ref()))
                .await?
                .finalize_extract_result(),
        )
    }
}
