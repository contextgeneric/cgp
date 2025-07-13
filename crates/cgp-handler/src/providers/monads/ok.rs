use crate::{Monadic, MonadicBind, Pure};

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
