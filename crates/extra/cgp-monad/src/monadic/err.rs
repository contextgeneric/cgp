use cgp::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

use crate::monadic::ident::IdentMonadic;
use crate::traits::{ContainsValue, LiftValue, MonadicBind, MonadicTrans};

/// The monad that continues on a step's Ok value and stops on the first Err, as `?` does;
/// PipeMonadic folds a pipeline under it with BindErr.
///
/// See [`ErrMonadic`](https://contextgeneric.dev/docs/reference/providers/monad#the-monad-markers).
pub struct ErrMonadic;

/// The `Err` monad as a transformer over a base monad `M`, so monads stack over nested results.
///
/// See [`ErrMonadicTrans`](https://contextgeneric.dev/docs/reference/providers/monad#the-monad-markers).
pub struct ErrMonadicTrans<M>(pub PhantomData<M>);

impl<M> MonadicTrans<M> for ErrMonadic {
    type M = ErrMonadicTrans<M>;
}

impl<M1, M2, M3> MonadicTrans<M2> for ErrMonadicTrans<M1>
where
    M1: MonadicTrans<M2, M = M3>,
{
    type M = ErrMonadicTrans<M3>;
}

impl<M, Provider> MonadicBind<Provider> for ErrMonadicTrans<M>
where
    M: MonadicBind<BindErr<M, Provider>>,
{
    type Provider = M::Provider;
}

impl<Provider> MonadicBind<Provider> for ErrMonadic {
    type Provider = BindErr<IdentMonadic, Provider>;
}
/// The per-step provider of the err monad: run the continuation on a step's Ok value and stop on
/// Err. PipeMonadic composes it under ErrMonadic.
///
/// See [`BindErr`](https://contextgeneric.dev/docs/reference/providers/monad/bind_err).
pub struct BindErr<M, Cont>(pub PhantomData<(M, Cont)>);

impl<T, E> ContainsValue<Result<T, E>> for ErrMonadic {
    type Value = T;
}

impl<T, E> LiftValue<T, Result<T, E>> for ErrMonadic {
    type Output = Result<T, E>;

    fn lift_value(value: T) -> Self::Output {
        Ok(value)
    }

    fn lift_output(output: Result<T, E>) -> Self::Output {
        output
    }
}

impl<T, E, V, M> ContainsValue<V> for ErrMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>>,
{
    type Value = T;
}

impl<T, E, V, M> LiftValue<T, V> for ErrMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>> + LiftValue<Result<T, E>, V>,
{
    type Output = M::Output;

    fn lift_value(value: T) -> Self::Output {
        M::lift_value(Ok(value))
    }

    fn lift_output(output: V) -> Self::Output {
        M::lift_output(output)
    }
}

#[cgp_provider]
impl<Context, Code, T1, T2, E, M, Cont> Computer<Context, Code, Result<T1, E>> for BindErr<M, Cont>
where
    Cont: Computer<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>> + LiftValue<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T1, E>) -> Self::Output {
        match input {
            Ok(value) => M::lift_output(Cont::compute(context, code, value)),
            Err(err) => M::lift_value(Err(err)),
        }
    }
}

#[cgp_provider]
impl<Context, Code, T1, T2, E, M, Cont> AsyncComputer<Context, Code, Result<T1, E>>
    for BindErr<M, Cont>
where
    Cont: AsyncComputer<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>> + LiftValue<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T1, E>,
    ) -> Self::Output {
        match input {
            Ok(value) => M::lift_output(Cont::compute_async(context, code, value).await),
            Err(err) => M::lift_value(Err(err)),
        }
    }
}
