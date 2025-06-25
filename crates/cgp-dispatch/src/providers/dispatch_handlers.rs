use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct DispatchHandlers<Handlers>(pub PhantomData<Handlers>);
pub struct DispatchHandlersRef<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Input: HasExtractor,
    Handlers: DispatchComputer<Context, Code, Input::Extractor, Output = Output>,
    Handlers::Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(_context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        let res = Handlers::compute(_context, code, input.to_extractor());

        match res {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> TryComputer<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractor,
    Handlers: TryDispatchComputer<Context, Code, Input::Extractor, Output = Output>,
    Handlers::Remainder: FinalizeExtract,
{
    type Output = Output;

    fn try_compute(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let res = Handlers::try_compute(_context, code, input.to_extractor())?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Handlers> Handler<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Context: HasAsyncErrorType,
    Input: HasExtractor,
    Handlers: DispatchHandler<Context, Code, Input::Extractor, Output = Output>,
    Handlers::Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let res = Handlers::handle(_context, code, input.to_extractor()).await?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Err(remainder.finalize_extract()),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> TryComputer<Context, Code, &Input>
    for DispatchHandlersRef<Handlers>
where
    Context: HasErrorType,
    Input: HasExtractorRef,
    Handlers: for<'b> TryDispatchComputer<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output = Output,
        Remainder: FinalizeExtract,
    >,
{
    type Output = Output;

    fn try_compute(
        _context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        let res = Handlers::try_compute(_context, code, input.extractor_ref())?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}
#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, &Input>
    for DispatchHandlersRef<Handlers>
where
    Input: HasExtractorRef,
    Handlers: for<'b> DispatchComputer<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output = Output,
        Remainder: FinalizeExtract,
    >,
{
    type Output = Output;

    fn compute(_context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        let res = Handlers::compute(_context, code, input.extractor_ref());

        match res {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input, Output: Send, Handlers> Handler<Context, Code, &Input>
    for DispatchHandlersRef<Handlers>
where
    Context: HasAsyncErrorType,
    Input: HasExtractorRef + Send + Sync,
    Handlers: for<'b> DispatchHandler<
        Context,
        Code,
        Input::ExtractorRef<'b>,
        Output = Output,
        Remainder: FinalizeExtract,
    >,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        let res = Handlers::handle(_context, code, input.extractor_ref()).await?;

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Err(remainder.finalize_extract()),
        }
    }
}

trait DispatchComputer<Context, Code, Input> {
    type Output;

    type Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder>;
}

trait TryDispatchComputer<Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    type Remainder;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error>;
}

impl<
        Context,
        Code,
        Input,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output,
        RemainderA,
        RemainderB,
    > DispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: Computer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        DispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        let res = CurrentHandler::compute(context, tag, input);

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Cons::compute(context, tag, remainder),
        }
    }
}

impl<
        Context,
        Code,
        Input,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output,
        RemainderA,
        RemainderB,
    > TryDispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasErrorType,
    CurrentHandler: TryComputer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        TryDispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        let res = CurrentHandler::try_compute(context, tag, input)?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::try_compute(context, tag, remainder),
        }
    }
}

impl<Context, Code, Input, Handler, Remainder, Output> DispatchComputer<Context, Code, Input>
    for Cons<Handler, Nil>
where
    Handler: Computer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        Handler::compute(context, tag, input)
    }
}

impl<Context, Code, Input, Handler, Remainder, Output> TryDispatchComputer<Context, Code, Input>
    for Cons<Handler, Nil>
where
    Context: HasErrorType,
    Handler: TryComputer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        Handler::try_compute(context, tag, input)
    }
}

#[async_trait]
trait DispatchHandler<Context, Code, Input>
where
    Context: HasAsyncErrorType,
{
    type Output: Send;

    type Remainder: Send;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error>;
}

impl<
        Context,
        Code: Send,
        Input: Send,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output: Send,
        RemainderA: Send,
        RemainderB: Send,
    > DispatchHandler<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        DispatchHandler<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        let res = CurrentHandler::handle(context, tag, input).await?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::handle(context, tag, remainder).await,
        }
    }
}

impl<Context, Code: Send, Input: Send, CurrentHandler, Remainder: Send, Output: Send>
    DispatchHandler<Context, Code, Input> for Cons<CurrentHandler, Nil>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        CurrentHandler::handle(context, tag, input).await
    }
}
