use core::marker::PhantomData;

use crate::traits::{CanWrap, ContainsValue, Functorial, MonadicBind, Pure};

pub struct ErrMonadic;

impl<T, E> ContainsValue<Result<T, E>> for ErrMonadic {
    type Value = T;
}

impl<T, E> CanWrap<Result<T, E>> for ErrMonadic {
    fn wrap(value: T) -> Result<T, E> {
        Ok(value)
    }
}

impl<T1, T2, E> Functorial<Result<T1, E>, T2> for ErrMonadic {
    type Output = Result<T2, E>;

    fn map(value: Result<T1, E>, cont: impl Fn(T1) -> T2) -> Result<T2, E> {
        value.map(cont)
    }
}

impl<T1, T2, E, N> MonadicBind<Result<T1, E>, N> for ErrMonadic
where
    N: IntoErr<E, T = T2>,
{
    type Output = Result<T2, E>;

    fn bind(value: Result<T1, E>, cont: impl Fn(T1) -> N) -> Result<T2, E> {
        cont(value?).into_err()
    }
}

pub struct ErrMonadicTrans<M>(pub PhantomData<M>);

impl<M, V, T, E> ContainsValue<V> for ErrMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>>,
{
    type Value = T;
}

trait IntoErr<E> {
    type T;

    fn into_err(self) -> Result<Self::T, E>;
}

impl<T, E> IntoErr<E> for Result<T, E> {
    type T = T;

    fn into_err(self) -> Result<Self::T, E> {
        self
    }
}

impl<T, E> IntoErr<E> for Pure<T> {
    type T = T;

    fn into_err(self) -> Result<Self::T, E> {
        Ok(self.0)
    }
}
