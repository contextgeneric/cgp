use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

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
    TryComputer<Context, Code, (&'a mut Input, Args)> for MatchFirstWithHandlersMut<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorMut,
    DispatchMatchers<Handlers>: TryComputer<
        Context,
        Code,
        (Input::ExtractorMut<'a>, Args),
        Output = Result<Output, (Remainder, Args)>,
    >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a mut Input, Args),
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::try_compute(context, code, (input.extractor_mut(), args))?;
        match res {
            Ok(output) => Ok(output),
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<'a, Context, Code: Send, Input, Args: Send, Output, Remainder, Handlers>
    Handler<Context, Code, (&'a mut Input, Args)> for MatchFirstWithHandlersMut<Handlers>
where
    Context: HasAsyncErrorType,
    Input: Send + Sync + HasExtractorMut,
    DispatchMatchers<Handlers>: Handler<
        Context,
        Code,
        (Input::ExtractorMut<'a>, Args),
        Output = Result<Output, (Remainder, Args)>,
    >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (&'a mut Input, Args),
    ) -> Result<Output, Context::Error> {
        let res = DispatchMatchers::handle(context, code, (input.extractor_mut(), args)).await?;

        match res {
            Ok(output) => Ok(output),
            Err((remainder, _)) => remainder.finalize_extract(),
        }
    }
}
