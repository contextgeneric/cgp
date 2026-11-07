use core::fmt::Debug;
use core::marker::PhantomData;

use crate::HasErrorType;

/// A context that carries only an error type.
///
/// See [`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type).
pub struct ErrorOnly<E>(pub PhantomData<E>);

impl<E> Default for ErrorOnly<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E: Debug> HasErrorType for ErrorOnly<E> {
    type Error = E;
}
