use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

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
    TryComputer<Context, Code, (&'a Input, Args)> for MatchFirstWithHandlersRef<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorRef,
    DispatchMatchers<Handlers>: TryComputer<
        Context,
        Code,
        (Input::ExtractorRef<'a>, Args),
        Output = Result<Output, (Remainder, Args)>,
    >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a Input, Args),
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::try_compute(context, code, (input.extractor_ref(), args))?;
        match res {
            Ok(output) => Ok(output),
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<'a, Context, Code: Send, Input, Args: Send, Output, Remainder, Handlers>
    Handler<Context, Code, (&'a Input, Args)> for MatchFirstWithHandlersRef<Handlers>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasExtractorRef,
    DispatchMatchers<Handlers>: Handler<
        Context,
        Code,
        (Input::ExtractorRef<'a>, Args),
        Output = Result<Output, (Remainder, Args)>,
    >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a Input, Args),
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::handle(context, code, (input.extractor_ref(), args)).await?;

        match res {
            Ok(output) => Ok(output),
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}
