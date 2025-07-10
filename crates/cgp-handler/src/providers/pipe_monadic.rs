use cgp_core::prelude::*;

use crate::{Computer, ComputerComponent};

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

#[cgp_provider]
impl<M, Context, Tag, Input, Output, Providers> Computer<Context, Tag, Input>
    for PipeMonadic<M, Providers>
where
    Providers: PipeComputer<M, Context, Tag, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Output {
        Providers::compute(context, tag, input)
    }
}

pub trait Monad {
    type M<T>;

    fn bind<T, U>(value: Self::M<T>, cont: impl Fn(T) -> Self::M<U>) -> Self::M<U>;
}

pub trait Monadic<T> {
    type Value;

    type Monad: Monad<M<Self::Value> = T>;
}

trait PipeComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

impl<M1, M2, Context, Tag, Input, Intermediary, Output, CurrentProvider, RestProviders>
    PipeComputer<M1, Context, Tag, Input> for Cons<CurrentProvider, RestProviders>
where
    M1: Monadic<CurrentProvider::Output, Value = Intermediary, Monad = M2>
        + Monadic<RestProviders::Output, Value = Output, Monad = M2>,
    M2: Monad<M<Intermediary> = CurrentProvider::Output> + Monad<M<Output> = RestProviders::Output>,
    CurrentProvider: Computer<Context, Tag, Input>,
    RestProviders: PipeComputer<M1, Context, Tag, Intermediary>,
{
    type Output = RestProviders::Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Self::Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        M2::bind::<Intermediary, Output>(intermediate, |intermediate| {
            RestProviders::compute(context, tag, intermediate)
        })
    }
}

impl<M, Context, Code, Input> PipeComputer<M, Context, Code, Input> for Nil {
    type Output = Input;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}
