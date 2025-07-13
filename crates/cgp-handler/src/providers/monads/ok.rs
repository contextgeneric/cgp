use core::marker::PhantomData;

use crate::{ContainsValue, MonadicBind, MonadicPure, MonadicTrans, Pure};

pub struct OkMonadic;

impl<T, E> ContainsValue<Result<T, E>> for OkMonadic {
    type Value = E;
}

impl<T, E> MonadicPure<Result<T, E>> for OkMonadic {
    fn pure(value: E) -> Result<T, E> {
        Err(value)
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

impl<M, T, E1, E2, E3, Next> MonadicBind<Result<T, E1>, Result<Next, E3>> for OkMonadicTrans<M>
where
    Next: IntoOk<T, E = E2>,
{
    type Output = Result<Result<T, E2>, E3>;

    fn bind(
        value: Result<T, E1>,
        cont: impl Fn(E1) -> Result<Next, E3>,
    ) -> Result<Result<T, E2>, E3> {
        todo!()
    }
}

// impl<M, T, E1, E2, E3> MonadicBind<Result<T, E1>, Result<Pure<E2>, E3>> for OkMonadicTrans<M> {
//     type Output = Result<Result<T, E2>, E3>;

//     fn bind(
//         value: Result<T, E1>,
//         cont: impl Fn(E1) -> Result<Pure<E2>, E3>,
//     ) -> Result<Result<T, E2>, E3> {
//         todo!()
//     }
// }

// impl<M, T, E> ContainsValue<Result<T, E>> for OkMonadicTrans<M> {
//     type Value = E;
// }

// impl<M, N, T, E1, E2, V, Out> MonadicBind<Result<T, E1>, N> for OkMonadicTrans<M>
// where
//     M: ContainsValue<N, Value = V>
//         + MonadicBind<N, Result<T, E2>, Output = Out>
//         + MonadicPure<Out, Value = Result<T, E2>>,
//     V: IntoOk<T, E = E2>,
// {
//     type Output = Out;

//     fn bind(value: Result<T, E1>, cont: impl Fn(E1) -> N) -> Out {
//         match value {
//             Ok(value) => M::pure(Ok(value)),
//             Err(err) => {
//                 let res = cont(err);
//                 M::bind(res, |value| value.into_ok())
//             }
//         }
//     }
// }

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
