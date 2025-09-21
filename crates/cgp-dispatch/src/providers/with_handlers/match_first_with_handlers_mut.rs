use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

use crate::DispatchMatchers;

pub struct MatchFirstWithHandlersMut<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<'a, Context, Code, Input, Args, Output, Remainder, Handlers>
    Computer<Context, Code, (&'a mut Input, Args)> for MatchFirstWithHandlersMut<Handlers>
where
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>: Computer<
            Context,
            Code,
            (Input::ExtractorMut<'a>, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a mut Input, Args),
    ) -> Output {
        let res = DispatchMatchers::compute(context, code, (input.extractor_mut(), args));
        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<'a, Context, Code, Input, Args, Output, Remainder, Handlers>
    AsyncComputer<Context, Code, (&'a mut Input, Args)> for MatchFirstWithHandlersMut<Handlers>
where
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>: AsyncComputer<
            Context,
            Code,
            (Input::ExtractorMut<'a>, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a mut Input, Args),
    ) -> Output {
        let res =
            DispatchMatchers::compute_async(context, code, (input.extractor_mut(), args)).await;

        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}
