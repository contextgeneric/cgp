use crate::{Monadic, MonadicBind, Pure};

pub struct ErrMonadic;

impl<T, E> Monadic<Result<T, E>> for ErrMonadic {
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
