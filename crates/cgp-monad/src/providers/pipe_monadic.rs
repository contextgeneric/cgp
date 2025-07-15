use std::marker::PhantomData;

use cgp_core::field::Cons;
use cgp_handler::Computer;

use crate::traits::{CanBindValue, Monadic};

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

trait MonadicComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, input: Input) -> Self::Output;
}

impl<Context, Code, M1, M2, Input, Intermediary, Output, ProviderA, ProviderB, RestProviders>
    MonadicComputer<M1, Context, Code, Input> for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    ProviderA: Computer<Context, Code, Input>,
    M1: Monadic<ProviderA::Output, Value = Intermediary, Monad = M2> + Monadic<Output, Monad = M2>,
    M2: CanBindValue,
    Cons<ProviderB, RestProviders>: Computer<Context, Code, Intermediary, Output = Output>,
{
    type Output = ();

    fn compute(context: &Context, input: Input) -> Self::Output {
        let value = ProviderA::compute(context, PhantomData, input);

        todo!()
    }
}
