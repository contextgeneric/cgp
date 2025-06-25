use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct BuildWithHandlers<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers> Computer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    Handlers: BuilderComputer<Context, Code, Builder>,
    Handlers::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        Handlers::compute(context, code, Output::builder()).finalize_build()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers> TryComputer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Context: HasErrorType,
    Output: HasBuilder<Builder = Builder>,
    Handlers: TryBuilderComputer<Context, Code, Builder>,
    Handlers::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(Handlers::try_compute(context, code, Output::builder())?.finalize_build())
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Builder, Handlers>
    Handler<Context, Code, Input> for BuildWithHandlers<Output, Handlers>
where
    Context: HasAsyncErrorType,
    Output: HasBuilder<Builder = Builder>,
    Handlers: BuilderHandler<Context, Code, Builder>,
    Handlers::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(Handlers::handle(context, code, Output::builder())
            .await?
            .finalize_build())
    }
}

trait BuilderComputer<Context, Code, Builder> {
    type Output;

    fn compute(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output;
}

trait TryBuilderComputer<Context, Code, Builder>
where
    Context: HasErrorType,
{
    type Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error>;
}

#[async_trait]
trait BuilderHandler<Context, Code, Builder>
where
    Context: HasAsyncErrorType,
{
    type Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error>;
}

impl<Context, Code, Builder, NextBuilder, Output, CurrentHandler, NextHandler, RestHandlers>
    BuilderComputer<Context, Code, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: Computer<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>: BuilderComputer<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let next_builder = CurrentHandler::compute(context, code, builder);
        Cons::compute(context, code, next_builder)
    }
}

impl<Context, Code, Builder, NextBuilder, Output, CurrentHandler, NextHandler, RestHandlers>
    TryBuilderComputer<Context, Code, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasErrorType,
    CurrentHandler: TryComputer<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>:
        TryBuilderComputer<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let next_builder = CurrentHandler::try_compute(context, code, builder)?;
        Cons::try_compute(context, code, next_builder)
    }
}

impl<
        Context,
        Code: Send,
        Builder: Send,
        NextBuilder,
        Output,
        CurrentHandler,
        NextHandler,
        RestHandlers,
    > BuilderHandler<Context, Code, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>: BuilderHandler<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let next_builder = CurrentHandler::handle(context, code, builder).await?;
        Cons::handle(context, code, next_builder).await
    }
}

impl<Context, Code, Builder, Handler, Output> BuilderComputer<Context, Code, Builder>
    for Cons<Handler, Nil>
where
    Handler: Computer<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        Handler::compute(context, code, builder)
    }
}

impl<Context, Code, Builder, Handler, Output> TryBuilderComputer<Context, Code, Builder>
    for Cons<Handler, Nil>
where
    Context: HasErrorType,
    Handler: TryComputer<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        Handler::try_compute(context, code, builder)
    }
}

impl<Context, Code: Send, Builder: Send, CurrentHandler, Output>
    BuilderHandler<Context, Code, Builder> for Cons<CurrentHandler, Nil>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        CurrentHandler::handle(context, code, builder).await
    }
}
