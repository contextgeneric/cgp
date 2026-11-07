use core::ops::Deref;

/// A value that is either a borrow of a T or an owned T, so one getter signature serves both the
/// context that stores a value and the one that computes it.
///
/// See [`MRef`](https://contextgeneric.dev/docs/reference/types/mref).
pub enum MRef<'a, T> {
    /// A borrowed value.
    Ref(&'a T),
    /// An owned value.
    Owned(T),
}

impl<T> Deref for MRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Ref(value) => value,
            Self::Owned(value) => value,
        }
    }
}

impl<T> AsRef<T> for MRef<'_, T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> From<T> for MRef<'_, T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

impl<'a, T> From<&'a T> for MRef<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Ref(value)
    }
}

impl<T> MRef<'_, T>
where
    T: Clone,
{
    /// Returns the value, cloning it when it is borrowed.
    pub fn get_or_clone(self) -> T {
        match self {
            Self::Ref(value) => value.clone(),
            Self::Owned(value) => value,
        }
    }
}
