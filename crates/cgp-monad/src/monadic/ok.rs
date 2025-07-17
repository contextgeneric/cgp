use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

use crate::monadic::ident::IdentMonadic;
use crate::traits::{ContainsValue, LiftValue, MonadicBind, MonadicTrans};

pub struct OkMonadic;

pub struct OkMonadicTrans<M>(pub PhantomData<M>);

pub struct BindOk<M, Cont>(pub PhantomData<(M, Cont)>);

impl<M> MonadicTrans<M> for OkMonadic {
    type M = OkMonadicTrans<M>;
}

impl<M1, M2, M3> MonadicTrans<M2> for OkMonadicTrans<M1>
where
    M1: MonadicTrans<M2, M = M3>,
{
    type M = OkMonadicTrans<M3>;
}

impl<M, Provider> MonadicBind<Provider> for OkMonadicTrans<M>
where
    M: MonadicBind<BindOk<M, Provider>>,
{
    type Provider = M::Provider;
}

impl<Provider> MonadicBind<Provider> for OkMonadic {
    type Provider = BindOk<IdentMonadic, Provider>;
}

impl<T, E> ContainsValue<Result<T, E>> for OkMonadic {
    type Value = E;
}

impl<T, E> LiftValue<E, Result<T, E>> for OkMonadic {
    type Output = Result<T, E>;

    fn lift_value(value: E) -> Self::Output {
        Err(value)
    }

    fn lift_output(output: Result<T, E>) -> Self::Output {
        output
    }
}

impl<T, E, V, M> ContainsValue<V> for OkMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>>,
{
    type Value = E;
}

impl<T, E, V, M> LiftValue<E, V> for OkMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>> + LiftValue<Result<T, E>, V>,
{
    type Output = M::Output;

    fn lift_value(value: E) -> Self::Output {
        M::lift_value(Err(value))
    }

    fn lift_output(output: V) -> Self::Output {
        M::lift_output(output)
    }
}

#[cgp_provider]
impl<Context, Code, T, E1, E2, M, Cont> Computer<Context, Code, Result<T, E1>> for BindOk<M, Cont>
where
    Cont: Computer<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value = Result<T, E2>> + LiftValue<Result<T, E2>, Cont::Output>,
{
    type Output = M::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T, E1>) -> Self::Output {
        match input {
            Err(value) => M::lift_output(Cont::compute(context, code, value)),
            Ok(err) => M::lift_value(Ok(err)),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, T: Send, E1: Send, E2: Send, M, Cont>
    Handler<Context, Code, Result<T, E1>> for BindOk<M, Cont>
where
    Context: HasAsyncErrorType,
    Cont: Handler<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value = Result<T, E2>> + LiftValue<Result<T, E2>, Cont::Output>,
{
    type Output = M::Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T, E1>,
    ) -> Result<Self::Output, Context::Error> {
        match input {
            Err(value) => Ok(M::lift_output(Cont::handle(context, code, value).await?)),
            Ok(err) => Ok(M::lift_value(Ok(err))),
        }
    }
}
