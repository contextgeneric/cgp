use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

use crate::DispatchMatchers;

pub struct MatchFirstWithHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Args, Output, Remainder, Handlers> Computer<Context, Code, (Input, Args)>
    for MatchFirstWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>: Computer<
            Context,
            Code,
            (Input::Extractor, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, (input, args): (Input, Args)) -> Output {
        let res = DispatchMatchers::compute(context, code, (input.to_extractor(), args));
        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Output, Remainder, Handlers>
    AsyncComputer<Context, Code, (Input, Args)> for MatchFirstWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>: AsyncComputer<
            Context,
            Code,
            (Input::Extractor, Args),
            Output = Result<Output, (Remainder, Args)>,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Output {
        let res =
            DispatchMatchers::compute_async(context, code, (input.to_extractor(), args)).await;

        match res {
            Ok(output) => output,
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}
