use core::ops::Deref;

pub enum MRef<'a, T> {
    Ref(&'a T),
    Owned(T),
}

impl<'a, T> Deref for MRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Ref(value) => value,
            Self::Owned(value) => value,
        }
    }
}

impl<'a, T> AsRef<T> for MRef<'a, T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}
