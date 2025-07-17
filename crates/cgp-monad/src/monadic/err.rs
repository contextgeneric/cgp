use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

use crate::monadic::ident::IdentMonadic;
use crate::traits::{ContainsValue, LiftValue, MonadicBind, MonadicTrans};

pub struct ErrMonadic;

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
impl<Context, Code: Send, T1: Send, T2, E: Send, M, Cont> Handler<Context, Code, Result<T1, E>>
    for BindErr<M, Cont>
where
    Context: HasAsyncErrorType,
    Cont: Handler<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value = Result<T2, E>> + LiftValue<Result<T2, E>, Cont::Output>,
{
    type Output = M::Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T1, E>,
    ) -> Result<Self::Output, Context::Error> {
        match input {
            Ok(value) => Ok(M::lift_output(Cont::handle(context, code, value).await?)),
            Err(err) => Ok(M::lift_value(Err(err))),
        }
    }
}
