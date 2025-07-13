use core::marker::PhantomData;

use crate::{Monadic, MonadicBind, Pure};

pub struct ErrMonadic;

impl<T, E> Monadic<Result<T, E>> for ErrMonadic {
    type Value = T;
}

impl<T1, T2, E> MonadicBind<Result<T1, E>, Result<T2, E>> for ErrMonadic {
    type Output = Result<T2, E>;
    type OutValue = T2;

    fn bind(value: Result<T1, E>, cont: impl Fn(T1) -> Result<T2, E>) -> Result<T2, E> {
        cont(value?)
    }

    fn pure(value: T2) -> Result<T2, E> {
        Ok(value)
    }
}

impl<T1, T2, E> MonadicBind<Result<T1, E>, Pure<T2>> for ErrMonadic {
    type Output = Result<T2, E>;
    type OutValue = T2;

    fn bind(value: Result<T1, E>, cont: impl Fn(T1) -> Pure<T2>) -> Result<T2, E> {
        Ok(cont(value?).0)
    }

    fn pure(value: T2) -> Pure<T2> {
        Pure(value)
    }
}

pub struct ErrMonadicTrans<M>(pub PhantomData<M>);

impl<M, V, T, E> Monadic<V> for ErrMonadicTrans<M>
where
    M: Monadic<V, Value = Result<T, E>>,
{
    type Value = T;
}

impl<M, V, N, T1, T2, E> MonadicBind<V, N> for ErrMonadicTrans<M>
where
    M: MonadicBind<V, N, Value = Result<T1, E>, OutValue = Result<T2, E>>,
{
    type Output = M::Output;
    type OutValue = T2;

    fn bind(value: V, cont: impl Fn(T1) -> N) -> M::Output {
        M::bind(value, |res| match res {
            Ok(value) => cont(value),
            Err(err) => M::pure(Err(err)),
        })
    }

    fn pure(value: T2) -> N {
        M::pure(Ok(value))
    }
}
