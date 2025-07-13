use cgp_core::prelude::*;

use crate::{Computer, ComputerComponent, ErrMonadic, TryComputer, TryComputerComponent};

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

#[cgp_provider]
impl<M, Context, Tag, Input, Output, Providers> TryComputer<Context, Tag, Input>
    for PipeMonadic<M, Providers>
where
    Context: HasErrorType,
    Providers: PipeTryComputer<M, Context, Tag, Input, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Providers::try_compute(context, tag, input)
    }
}

pub trait Monadic<T> {
    type Value;
}

pub trait MonadicTrans<T> {
    type M;
}

pub trait MonadicBind<T, Next>: Monadic<T> {
    type Output;
    type OutValue;

    fn bind(wrapped: T, cont: impl Fn(Self::Value) -> Next) -> Self::Output;
    fn pure(value: Self::OutValue) -> Next;
}

trait PipeComputer<M, Context, Code, Input> {
    type Output;

    fn compute(context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

pub struct Pure<T>(pub T);

impl<M, Context, Tag, Input, Intermediary, CurrentProvider, RestProviders, Output>
    PipeComputer<M, Context, Tag, Input> for Cons<CurrentProvider, RestProviders>
where
    M: Monadic<CurrentProvider::Output, Value = Intermediary>
        + MonadicBind<CurrentProvider::Output, RestProviders::Output, Output = Output>,
    CurrentProvider: Computer<Context, Tag, Input>,
    RestProviders: PipeComputer<M, Context, Tag, Intermediary>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Tag>, input: Input) -> Self::Output {
        let intermediate = CurrentProvider::compute(context, tag, input);
        M::bind(intermediate, |intermediate| {
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

trait PipeTryComputer<M, Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    fn try_compute(
        context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error>;
}

impl<M1, M2, Context, Tag, Input, Intermediary, Output, CurrentProvider, RestProviders>
    PipeTryComputer<M1, Context, Tag, Input> for Cons<CurrentProvider, RestProviders>
where
    M1: MonadicTrans<ErrMonadic, M = M2>,
    // M2: Monadic<Result<CurrentProvider::Output, Context::Error>, Value = Intermediary>,
    //     + MonadicBind<
    //         Result<CurrentProvider::Output, Context::Error>,
    //         Result<RestProviders::Output, Context::Error>,
    //         // Output = Result<Output, Context::Error>,
    //     >
    // ,
    M1: Monadic<CurrentProvider::Output, Value = Intermediary>
        + MonadicBind<CurrentProvider::Output, RestProviders::Output, Output = Output>,
    Context: HasErrorType,
    CurrentProvider: TryComputer<Context, Tag, Input>,
    RestProviders: PipeTryComputer<M1, Context, Tag, Intermediary>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Tag>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let intermediate = CurrentProvider::try_compute(context, tag, input)?;
        todo!()
        // Ok(M1::bind(intermediate, |intermediate| {
        //     todo!()
        //     // RestProviders::try_compute(context, tag, intermediate)
        // }))
    }
}

impl<M, Context, Code, Input> PipeTryComputer<M, Context, Code, Input> for Nil
where
    Context: HasErrorType,
{
    type Output = Pure<Input>;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Pure<Input>, Context::Error> {
        Ok(Pure(input))
    }
}
