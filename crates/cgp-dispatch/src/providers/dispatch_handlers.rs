use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchMatchers;

pub struct DispatchHandlers<Handlers>(pub PhantomData<Handlers>);
pub struct DispatchHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Remainder, Handlers> Computer<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(_context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        let res = DispatchMatchers::compute(_context, code, input.to_extractor());

        match res {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Remainder, Handlers> TryComputer<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        TryComputer<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::try_compute(_context, code, input.to_extractor())?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Remainder: Send, Handlers>
    Handler<Context, Code, Input> for DispatchHandlers<Handlers>
where
    Context: HasAsyncErrorType,
    Input: HasExtractor<Extractor: Send>,
    DispatchMatchers<Handlers>:
        Handler<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::handle(_context, code, input.to_extractor()).await?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Err(remainder.finalize_extract()),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, &Input>
    for DispatchHandlersRef<Handlers>
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
    for DispatchHandlersRef<Handlers>
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
    for DispatchHandlersRef<Handlers>
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
