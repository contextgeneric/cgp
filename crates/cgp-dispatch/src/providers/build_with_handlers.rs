use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::DispatchBuilders;

pub struct BuildWithHandlers<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> Computer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    DispatchBuilders<Handlers>: Computer<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        DispatchBuilders::compute(context, code, Output::builder()).finalize_build()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> TryComputer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Context: HasErrorType,
    Output: HasBuilder<Builder = Builder>,
    DispatchBuilders<Handlers>: TryComputer<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(DispatchBuilders::try_compute(context, code, Output::builder())?.finalize_build())
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, Builder: Send, Handlers, Res>
    Handler<Context, Code, Input> for BuildWithHandlers<Output, Handlers>
where
    Context: HasAsyncErrorType,
    Output: HasBuilder<Builder = Builder>,
    DispatchBuilders<Handlers>: Handler<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(DispatchBuilders::handle(context, code, Output::builder())
            .await?
            .finalize_build())
    }
}
