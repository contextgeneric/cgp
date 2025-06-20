use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

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

trait BuilderComputer<Context, Code, Builder> {
    type Output;

    fn build(context: &Context, code: PhantomData<Code>, input: Builder) -> Self::Output;
}

impl<Context, Code, Builder, NextBuilder, Output, CurrentHandler, NextHandler, RestHandlers>
    BuilderComputer<Context, Code, Builder>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: Computer<Context, Code, Builder, Output = NextBuilder>,
    Cons<NextHandler, RestHandlers>: BuilderComputer<Context, Code, NextBuilder, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let next_builder = CurrentHandler::compute(context, code, builder);
        Cons::build(context, code, next_builder)
    }
}

impl<Context, Code, Builder, Handler, Output> BuilderComputer<Context, Code, Builder>
    for Cons<Handler, Nil>
where
    Handler: Computer<Context, Code, Builder, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        Handler::compute(context, code, builder)
    }
}
