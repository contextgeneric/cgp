use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, M, Providers> Computer<Context, Code, Input>
    for PipeMonadic<M, Providers>
where
    Providers: PipeComputer<M, Context, Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output {
        Providers::compute(context, input)
    }
}

trait PipeComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, input: Input) -> Self::Output;
}

impl<Context, Code, Input, M, ProviderA, ProviderB, RestProviders, OutProvider>
    PipeComputer<M, Context, Code, Input> for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    M: MonadicBind<
        ProviderA,
        PipeMonadic<M, Cons<ProviderB, RestProviders>>,
        Provider = OutProvider,
    >,
    OutProvider: Computer<Context, Code, Input>,
{
    type Output = OutProvider::Output;

    fn compute(context: &Context, input: Input) -> Self::Output {
        OutProvider::compute(context, PhantomData, input)
    }
}

impl<Context, Code, Input, M, Provider> PipeComputer<M, Context, Code, Input>
    for Cons<Provider, Nil>
where
    Provider: Computer<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn compute(context: &Context, input: Input) -> Self::Output {
        Provider::compute(context, PhantomData, input)
    }
}

// struct Bind<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

// impl<M, Context, Code, Input, ProviderA, ProviderB, ProviderC> PipeComputer<M, Context, Code, Input>
//     for Bind<ProviderA, ProviderB>
// where
//     M: MonadicBind<ProviderA, ProviderB, Provider = ProviderC>,
//     ProviderC: Computer<Context, Code, Input>,
// {
//     type Output = ProviderC::Output;

//     fn compute(context: &Context, input: Input) -> Self::Output {
//         ProviderC::compute(context, PhantomData, input)
//     }
// }

// impl<M, Context, Code, Input, ProviderA, ProviderB> PipeComputer<M,
