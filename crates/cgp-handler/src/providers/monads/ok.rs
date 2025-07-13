use core::marker::PhantomData;

use crate::{Monadic, MonadicBind, MonadicTrans, Pure};

pub struct OkMonadic;

impl<T, E> Monadic<Result<T, E>> for OkMonadic {
    type Value = E;
}

impl<T, E1, E2> MonadicBind<Result<T, E1>, Result<T, E2>> for OkMonadic {
    type Output = Result<T, E2>;
    type OutValue = E2;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> Result<T, E2>) -> Result<T, E2> {
        match value {
            Ok(value) => Ok(value),
            Err(err) => cont(err),
        }
    }

    fn pure(value: E2) -> Result<T, E2> {
        Err(value)
    }
}

impl<T, E1, E2> MonadicBind<Result<T, E1>, Pure<E2>> for OkMonadic {
    type Output = Result<T, E2>;
    type OutValue = E2;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> Pure<E2>) -> Result<T, E2> {
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(cont(err).0),
        }
    }

    fn pure(value: E2) -> Pure<E2> {
        Pure(value)
    }
}

impl<M> MonadicTrans<M> for OkMonadic {
    type M = OkMonadicTrans<M>;
}

pub struct OkMonadicTrans<M>(pub PhantomData<M>);

impl<M, V, T, E> Monadic<V> for OkMonadicTrans<M>
where
    M: Monadic<V, Value = Result<T, E>>,
{
    type Value = E;
}

impl<M, V, N, T, E1, E2> MonadicBind<V, N> for OkMonadicTrans<M>
where
    M: MonadicBind<V, N, Value = Result<T, E1>, OutValue = Result<T, E2>>,
{
    type Output = M::Output;
    type OutValue = E2;

    fn bind(value: V, cont: impl Fn(E1) -> N) -> M::Output {
        M::bind(value, |res| match res {
            Err(err) => cont(err),
            Ok(value) => M::pure(Ok(value)),
        })
    }

    fn pure(err: E2) -> N {
        M::pure(Err(err))
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
