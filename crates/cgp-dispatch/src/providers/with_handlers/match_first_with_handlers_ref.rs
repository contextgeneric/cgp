use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

use crate::DispatchMatchers;

pub struct MatchFirstWithHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<'a, Context, Code, Input, Args, Output, Remainder, Handlers>
    Computer<Context, Code, (&'a Input, Args)> for MatchFirstWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>: Computer<
            Context,
            Code,
            (Input::ExtractorRef<'a>, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a Input, Args),
    ) -> Output {
        let res = DispatchMatchers::compute(context, code, (input.extractor_ref(), args));
        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<'a, Context, Code, Input, Args, Output, Remainder, Handlers>
    AsyncComputer<Context, Code, (&'a Input, Args)> for MatchFirstWithHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>: AsyncComputer<
            Context,
            Code,
            (Input::ExtractorRef<'a>, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a Input, Args),
    ) -> Output {
        let res =
            DispatchMatchers::compute_async(context, code, (input.extractor_ref(), args)).await;

        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}
