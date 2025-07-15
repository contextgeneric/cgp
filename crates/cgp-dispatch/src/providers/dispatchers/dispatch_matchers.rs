use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{ComputerComponent, Handler, HandlerComponent, TryComputerComponent};
use cgp_monad::{monads::ok::OkMonadic, providers::PipeMonadic};

pub struct DispatchMatchers<Handlers>(pub PhantomData<Handlers>);

delegate_components! {
    <Providers>
    DispatchMatchers<Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
        ]:
            PipeMonadic<OkMonadic, Providers>,
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
