use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, TryComputer, TryComputerComponent};

pub struct BuildWithHandlers<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers> Computer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    Handlers: BuilderComputer<Context, Code, Input, Builder>,
    Handlers::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Handlers::build(context, code, input, Output::builder()).finalize_build()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers> TryComputer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Context: HasErrorType,
    Output: HasBuilder<Builder = Builder>,
    Handlers: TryBuilderComputer<Context, Code, Input, Builder>,
    Handlers::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(Handlers::try_build(context, code, input, Output::builder())?.finalize_build())
    }
}

pub trait BuilderComputer<Context, Code, Input, Builder> {
    type Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output;
}

pub trait TryBuilderComputer<Context, Code, Input, Builder>
where
    Context: HasErrorType,
{
    type Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error>;
}

impl<
        Context,
        Code,
        Input,
        Builder,
        NextBuilder,
        Output,
        CurrentHandler,
        NextHandler,
        RestHandlers,
    > BuilderComputer<Context, Code, Input, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: BuilderComputer<Context, Code, Input, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>:
        BuilderComputer<Context, Code, Input, NextBuilder, Output = Output>,
    Input: Clone,
{
    type Output = Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output {
        let next_builder = CurrentHandler::build(context, code, input.clone(), builder);
        Cons::build(context, code, input, next_builder)
    }
}

impl<
        Context,
        Code,
        Input,
        Builder,
        NextBuilder,
        Output,
        CurrentHandler,
        NextHandler,
        RestHandlers,
    > TryBuilderComputer<Context, Code, Input, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasErrorType,
    CurrentHandler: TryBuilderComputer<Context, Code, Input, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>:
        TryBuilderComputer<Context, Code, Input, NextBuilder, Output = Output>,
    Input: Clone,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let next_builder = CurrentHandler::try_build(context, code, input.clone(), builder)?;
        Cons::try_build(context, code, input, next_builder)
    }
}

impl<Context, Code, Input, Builder, Handler, Output> BuilderComputer<Context, Code, Input, Builder>
    for Cons<Handler, Nil>
where
    Handler: BuilderComputer<Context, Code, Input, Builder, Output = Output>,
{
    type Output = Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output {
        Handler::build(context, code, input, builder)
    }
}

impl<Context, Code, Input, Builder, Handler, Output>
    TryBuilderComputer<Context, Code, Input, Builder> for Cons<Handler, Nil>
where
    Context: HasErrorType,
    Handler: TryBuilderComputer<Context, Code, Input, Builder, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        Handler::try_build(context, code, input, builder)
    }
}
