use core::marker::PhantomData;

use crate::core::field::traits::FinalizeExtractResult;
use crate::core::prelude::*;
use crate::extra::dispatch::DispatchMatchers;
use crate::extra::handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

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
    AsyncComputer<Context, Code, &'a mut Input> for MatchWithHandlersMut<Handlers>
where
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>:
        AsyncComputer<Context, Code, Input::ExtractorMut<'a>, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        input: &'a mut Input,
    ) -> Output {
        DispatchMatchers::compute_async(context, code, input.extractor_mut())
            .await
            .finalize_extract_result()
    }
}
