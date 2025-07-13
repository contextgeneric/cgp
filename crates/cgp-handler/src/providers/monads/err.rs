use core::marker::PhantomData;

use crate::{ContainsValue, MonadicBind, Pure};

pub struct ErrMonadic;

impl<T, E> ContainsValue<Result<T, E>> for ErrMonadic {
    type Value = T;
}

impl<T1, T2, E> MonadicBind<Result<T1, E>, Result<T2, E>> for ErrMonadic {
    type Output = Result<T2, E>;

    fn bind(value: Result<T1, E>, cont: impl Fn(T1) -> Result<T2, E>) -> Result<T2, E> {
        cont(value?)
    }
}

impl<T1, T2, E> MonadicBind<Result<T1, E>, Pure<T2>> for ErrMonadic {
    type Output = Result<T2, E>;

    fn bind(value: Result<T1, E>, cont: impl Fn(T1) -> Pure<T2>) -> Result<T2, E> {
        Ok(cont(value?).0)
    }
}

pub struct ErrMonadicTrans<M>(pub PhantomData<M>);

impl<M, V, T, E> ContainsValue<V> for ErrMonadicTrans<M>
where
    M: ContainsValue<V, Value = Result<T, E>>,
{
    type Value = T;
}

// impl<M, V, N, T1, E> MonadicBind<V, N> for ErrMonadicTrans<M>
// where
//     M: MonadicBind<V, N, Value = Result<T1, E>, OutValue = Result<T2, E>>,
// {
//     type Output = M::Output;

//     fn bind(value: V, cont: impl Fn(T1) -> N) -> M::Output {
//         M::bind(value, |res| match res {
//             Ok(value) => cont(value),
//             Err(err) => M::pure(Err(err)),
//         })
//     }

//     fn pure(value: T2) -> N {
//         M::pure(Ok(value))
//     }
// }
