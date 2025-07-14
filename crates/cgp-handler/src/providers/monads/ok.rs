use core::future::Future;

use cgp_core::prelude::*;

use crate::{
    CanWrap, ContainsValue, Functorial, MonadicBind, MonadicBindAsync, MonadicTrans, Pure,
};

pub struct OkMonadic;

impl<T, E> ContainsValue<Result<T, E>> for OkMonadic {
    type Value = E;
}

impl<T, E> CanWrap<Result<T, E>> for OkMonadic {
    fn wrap(value: E) -> Result<T, E> {
        Err(value)
    }
}

impl<T, E1, E2> Functorial<Result<T, E1>, E2> for OkMonadic {
    type Output = Result<T, E2>;

    fn map(value: Result<T, E1>, cont: impl Fn(E1) -> E2) -> Result<T, E2> {
        value.map_err(cont)
    }
}

impl<T, E1, E2> MonadicBind<Result<T, E1>, Result<T, E2>> for OkMonadic {
    type Output = Result<T, E2>;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> Result<T, E2>) -> Result<T, E2> {
        match value {
            Ok(value) => Ok(value),
            Err(err) => cont(err),
        }
    }
}

impl<T, E1, E2> MonadicBind<Result<T, E1>, Pure<E2>> for OkMonadic {
    type Output = Result<T, E2>;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> Pure<E2>) -> Result<T, E2> {
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(cont(err).0),
        }
    }
}

impl<M> MonadicTrans<M> for OkMonadic {
    type M = OkMonadicTrans<M>;
}

pub struct OkMonadicTrans<M>(pub PhantomData<M>);

impl<M, T, E> ContainsValue<Result<T, E>> for OkMonadicTrans<M> {
    type Value = E;
}

impl<M, Next, T, E1, E2, V, Out> MonadicBind<Result<T, E1>, Next> for OkMonadicTrans<M>
where
    M: ContainsValue<Next, Value = V>
        + Functorial<Next, Result<T, E2>, Output = Out>
        + CanWrap<Out, Value = Result<T, E2>>,
    V: IntoOk<T, E = E2>,
{
    type Output = Out;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> Next) -> Out {
        match value {
            Ok(value) => M::wrap(Ok(value)),
            Err(err) => {
                let res = cont(err);
                M::map(res, |value| value.into_ok())
            }
        }
    }
}

impl<M: Async, Next: Async, T: Async, E1: Async, E2: Async, V: Async, Out: Async>
    MonadicBindAsync<Result<T, E1>, Next> for OkMonadicTrans<M>
where
    M: ContainsValue<Next, Value = V>
        + Functorial<Next, Result<T, E2>, Output = Out>
        + CanWrap<Out, Value = Result<T, E2>>,
    V: IntoOk<T, E = E2>,
{
    type Output = Out;

    async fn bind<Cont, F>(value: Result<T, E1>, cont: Cont) -> Out
    where
        Cont: Fn(Self::Value) -> F + Send,
        F: Future<Output = Next> + Send,
    {
        match value {
            Ok(value) => M::wrap(Ok(value)),
            Err(err) => {
                let res = cont(err).await;
                M::map(res, |value| value.into_ok())
            }
        }
    }
}

trait IntoOk<T> {
    type E;

    fn into_ok(self) -> Result<T, Self::E>;
}

impl<T, E> IntoOk<T> for Result<T, E> {
    type E = E;

    fn into_ok(self) -> Result<T, E> {
        self
    }
}

impl<T, E> IntoOk<T> for Pure<E> {
    type E = E;

    fn into_ok(self) -> Result<T, E> {
        Err(self.0)
    }
}
