use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

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
impl<'a, Context, Code, Input, Output, Remainder, Handlers> AsyncComputer<Context, Code, &'a Input>
    for MatchWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>:
        AsyncComputer<Context, Code, Input::ExtractorRef<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(context: &Context, code: PhantomData<Code>, input: &'a Input) -> Output {
        DispatchMatchers::compute_async(context, code, input.extractor_ref())
            .await
            .finalize_extract_result()
    }
}
