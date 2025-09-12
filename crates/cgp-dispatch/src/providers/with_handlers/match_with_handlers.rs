use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

use crate::DispatchMatchers;

pub struct MatchWithHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Remainder, Handlers> Computer<Context, Code, Input>
    for MatchWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        DispatchMatchers::compute(context, code, input.to_extractor()).finalize_extract_result()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Remainder, Handlers> AsyncComputer<Context, Code, Input>
    for MatchWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        AsyncComputer<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        DispatchMatchers::compute_async(context, code, input.to_extractor())
            .await
            .finalize_extract_result()
    }
}
