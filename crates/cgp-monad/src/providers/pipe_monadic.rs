use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

#[cgp_provider]
impl<Context, Code, Input, M, ProviderA, ProviderB, RestProviders, OutProvider>
    Computer<Context, Code, Input>
    for PipeMonadic<M, Cons<ProviderA, Cons<ProviderB, RestProviders>>>
where
    M: MonadicBind<
        ProviderA,
        PipeMonadic<M, Cons<ProviderB, RestProviders>>,
        Provider = OutProvider,
    >,
    OutProvider: Computer<Context, Code, Input>,
{
    type Output = OutProvider::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        OutProvider::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, M, Provider> Computer<Context, Code, Input>
    for PipeMonadic<M, Cons<Provider, Nil>>
where
    Provider: Computer<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Provider::compute(context, code, input)
    }
}
