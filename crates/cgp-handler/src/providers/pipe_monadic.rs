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

pub trait Monadic<T> {
    type Value;
}

pub trait MonadicBind<T, Next>: Monadic<T> {
    type Output;

    fn bind(wrapped: T, cont: impl Fn(Self::Value) -> Next) -> Self::Output;
}

trait PipeComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

pub struct Pure<T>(pub T);

impl<M1, Context, Tag, Input, Intermediary, CurrentProvider, RestProviders, Output>
    PipeComputer<M1, Context, Tag, Input> for Cons<CurrentProvider, RestProviders>
where
    M1: Monadic<CurrentProvider::Output, Value = Intermediary>
        + MonadicBind<CurrentProvider::Output, RestProviders::Output, Output = Output>,
    CurrentProvider: Computer<Context, Tag, Input>,
    RestProviders: PipeComputer<M1, Context, Tag, Intermediary>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Self::Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        M1::bind(intermediate, |intermediate| {
            RestProviders::compute(context, tag, intermediate)
        })
    }
}

impl<M, Context, Code, Input> PipeComputer<M, Context, Code, Input> for Nil {
    type Output = Pure<Input>;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: Input) -> Pure<Input> {
        Pure(input)
    }
}
