use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::Compose;

pub struct IdentMonadic;

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for IdentMonadic {
    type Provider = ComposeIdent<ProviderA, ProviderB>;
}

pub struct ComposeIdent<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB> Computer<Context, Code, Input>
    for ComposeIdent<ProviderA, ProviderB>
where
    ProviderA: Computer<Context, Code, Input>,
    ProviderB: Computer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let res = ProviderA::compute(context, code, input);
        ProviderB::compute(context, code, res)
    }
}

#[cgp_provider]
impl<Context, Code, Input, ProviderA, ProviderB> TryComputer<Context, Code, Input>
    for ComposeIdent<ProviderA, ProviderB>
where
    Context: HasErrorType,
    ProviderA: TryComputer<Context, Code, Input>,
    ProviderB: TryComputer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::try_compute(context, code, input)?;
        ProviderB::try_compute(context, code, res)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, ProviderA, ProviderB> Handler<Context, Code, Input>
    for ComposeIdent<ProviderA, ProviderB>
where
    Context: HasAsyncErrorType,
    ProviderA: Handler<Context, Code, Input>,
    ProviderB: Handler<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let res = ProviderA::handle(context, code, input).await?;
        ProviderB::handle(context, code, res).await
    }
}
