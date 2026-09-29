use core::fmt::Debug;
use core::marker::PhantomData;

use crate::HasErrorType;

pub struct ErrorOnly<E>(pub PhantomData<E>);

impl<E> Default for ErrorOnly<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E: Debug> HasErrorType for ErrorOnly<E> {
    type Error = E;
}
