use cgp_core::field::{MapFields, MapType};
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::{BuildAndMerge, BuildWithHandlers};

pub struct BuildAndMergeOutputs<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, Input>
    for BuildAndMergeOutputs<Output, Handlers>
where
    Handlers: MapFields<ToBuildAndMergeHandler>,
    BuildWithHandlers<Output, Handlers::Mapped>: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        BuildWithHandlers::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> TryComputer<Context, Code, Input>
    for BuildAndMergeOutputs<Output, Handlers>
where
    Context: HasErrorType,
    Handlers: MapFields<ToBuildAndMergeHandler>,
    BuildWithHandlers<Output, Handlers::Mapped>: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        BuildWithHandlers::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output, Handlers> Handler<Context, Code, Input>
    for BuildAndMergeOutputs<Output, Handlers>
where
    Context: HasAsyncErrorType,
    Handlers: MapFields<ToBuildAndMergeHandler>,
    BuildWithHandlers<Output, Handlers::Mapped>: Handler<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        BuildWithHandlers::handle(context, code, input).await
    }
}

struct ToBuildAndMergeHandler;

impl MapType for ToBuildAndMergeHandler {
    type Mapped<Handler> = BuildAndMerge<Handler>;
}
