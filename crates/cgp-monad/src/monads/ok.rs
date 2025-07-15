use core::marker::PhantomData;

use cgp_core::field::MapType;

use crate::traits::{CanBindValue, CanWrapValue};

pub struct OkMonad<T>(pub PhantomData<T>);

impl<T> MapType for OkMonad<T> {
    type Map<E> = Result<T, E>;
}

impl<T> CanWrapValue for OkMonad<T> {
    fn wrap<E>(e: E) -> Result<T, E> {
        Err(e)
    }
}

impl<T> CanBindValue for OkMonad<T> {
    fn bind<E1, E2>(res: Result<T, E1>, f: impl Fn(E1) -> Result<T, E2>) -> Result<T, E2> {
        match res {
            Ok(value) => Ok(value),
            Err(err) => f(err),
        }
    }
}
