use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

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

pub trait BuilderComputer<Context, Code, Input, Builder> {
    type Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output;
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
