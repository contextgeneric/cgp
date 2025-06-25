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
        Handlers::build(context, code, Output::builder()).finalize_build()
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
        Ok(Handlers::try_build(context, code, Output::builder())?.finalize_build())
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

pub trait BuilderComputer<Context, Code, Builder> {
    type Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output;
}

pub trait TryBuilderComputer<Context, Code, Builder>
where
    Context: HasErrorType,
{
    type Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error>;
}

#[async_trait]
pub trait BuilderHandler<Context, Code, Builder>
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
    CurrentHandler: BuilderComputer<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>: BuilderComputer<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let next_builder = CurrentHandler::build(context, code, builder);
        Cons::build(context, code, next_builder)
    }
}

impl<Context, Code, Builder, NextBuilder, Output, CurrentHandler, NextHandler, RestHandlers>
    TryBuilderComputer<Context, Code, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasErrorType,
    CurrentHandler: TryBuilderComputer<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>:
        TryBuilderComputer<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let next_builder = CurrentHandler::try_build(context, code, builder)?;
        Cons::try_build(context, code, next_builder)
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
    CurrentHandler: BuilderHandler<Context, Code, Builder, Output = NextBuilder>,
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
    Handler: BuilderComputer<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        Handler::build(context, code, builder)
    }
}

impl<Context, Code, Builder, Handler, Output> TryBuilderComputer<Context, Code, Builder>
    for Cons<Handler, Nil>
where
    Context: HasErrorType,
    Handler: TryBuilderComputer<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        Handler::try_build(context, code, builder)
    }
}

impl<Context, Code: Send, Builder: Send, Handler, Output> BuilderHandler<Context, Code, Builder>
    for Cons<Handler, Nil>
where
    Context: HasAsyncErrorType,
    Handler: BuilderHandler<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        Handler::handle(context, code, builder).await
    }
}
