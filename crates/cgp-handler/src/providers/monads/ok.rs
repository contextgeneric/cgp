use core::marker::PhantomData;

use crate::{ContainsValue, MonadicBind, MonadicTrans, Pure};

pub struct OkMonadic;

impl<T, E> ContainsValue<Result<T, E>> for OkMonadic {
    type Value = E;
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

impl<M, N, T, E1> MonadicBind<Result<T, E1>, N> for OkMonadicTrans<M>
where
    M: ContainsValue<N>,
{
    type Output = N;

    fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> N) -> N {
        match value {
            Ok(value) => todo!(),
            Err(err) => cont(err),
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
