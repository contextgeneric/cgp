use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct DispatchMatchers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Providers, Output, Remainder> Computer<Context, Code, Input>
    for DispatchMatchers<Providers>
where
    Providers: DispatchComputer<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Providers::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Providers, Output, Remainder> TryComputer<Context, Code, Input>
    for DispatchMatchers<Providers>
where
    Context: HasErrorType,
    Providers: TryDispatchComputer<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Providers::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Providers, Output: Send, Remainder: Send>
    Handler<Context, Code, Input> for DispatchMatchers<Providers>
where
    Context: HasAsyncErrorType,
    Providers: DispatchHandler<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Providers::handle(context, code, input).await
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

#[async_trait]
trait DispatchHandler<Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    type Remainder;

    async fn handle(
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
        CurrentProvider,
        RestProviders,
        Output,
        RemainderA,
        RemainderB,
    > DispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<CurrentProvider, RestProviders>>
where
    CurrentHandler: Computer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<CurrentProvider, RestProviders>:
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
        CurrentProvider,
        NextProvider,
        RestProviders,
        Output,
        RemainderA,
        RemainderB,
    > TryDispatchComputer<Context, Code, Input>
    for Cons<CurrentProvider, Cons<NextProvider, RestProviders>>
where
    Context: HasErrorType,
    CurrentProvider: TryComputer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextProvider, RestProviders>:
        TryDispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Output, RemainderB>, Context::Error> {
        let res = CurrentProvider::try_compute(context, tag, input)?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::try_compute(context, tag, remainder),
        }
    }
}

impl<Context, Code, Input, Provider, Remainder, Output> DispatchComputer<Context, Code, Input>
    for Cons<Provider, Nil>
where
    Provider: Computer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        Provider::compute(context, tag, input)
    }
}

impl<Context, Code, Input, Provider, Remainder, Output> TryDispatchComputer<Context, Code, Input>
    for Cons<Provider, Nil>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}

impl<
        Context,
        Code: Send,
        Input: Send,
        CurrentProvider,
        NextProvider,
        RestProviders,
        Output: Send,
        RemainderA: Send,
        RemainderB: Send,
    > DispatchHandler<Context, Code, Input>
    for Cons<CurrentProvider, Cons<NextProvider, RestProviders>>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Handler<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextProvider, RestProviders>:
        DispatchHandler<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        let res = CurrentProvider::handle(context, tag, input).await?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::handle(context, tag, remainder).await,
        }
    }
}

impl<Context, Code: Send, Input: Send, CurrentProvider, Remainder: Send, Output: Send>
    DispatchHandler<Context, Code, Input> for Cons<CurrentProvider, Nil>
where
    Context: HasAsyncErrorType,
    CurrentProvider: Handler<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        CurrentProvider::handle(context, tag, input).await
    }
}
