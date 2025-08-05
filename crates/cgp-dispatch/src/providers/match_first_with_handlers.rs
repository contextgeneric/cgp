use core::marker::PhantomData;

use cgp_core::field::FinalizeExtractResult;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchMatchers;

pub struct MatchFirstWithHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Args, Output, Remainder, Handlers> Computer<Context, Code, (Input, Args)>
    for MatchFirstWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, (Input::Extractor, Args), Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, (input, args): (Input, Args)) -> Output {
        DispatchMatchers::compute(context, code, (input.to_extractor(), args))
            .finalize_extract_result()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Output, Remainder, Handlers>
    TryComputer<Context, Code, (Input, Args)> for MatchFirstWithHandlers<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        TryComputer<Context, Code, (Input::Extractor, Args), Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::try_compute(context, code, (input.to_extractor(), args))?
                .finalize_extract_result(),
        )
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Args: Send, Output: Send, Remainder: Send, Handlers>
    Handler<Context, Code, (Input, Args)> for MatchFirstWithHandlers<Handlers>
where
    Context: HasAsyncErrorType,
    Input: HasExtractor<Extractor: Send>,
    DispatchMatchers<Handlers>:
        Handler<Context, Code, (Input::Extractor, Args), Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Output, Context::Error> {
        Ok(
            DispatchMatchers::handle(context, code, (input.to_extractor(), args))
                .await?
                .finalize_extract_result(),
        )
    }
}
