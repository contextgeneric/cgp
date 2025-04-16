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

impl<'a, T> From<T> for MRef<'a, T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

impl<'a, T> From<&'a T> for MRef<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Ref(value)
    }
}

impl<'a, T> MRef<'a, T>
where
    T: Clone,
{
    pub fn get_or_clone(self) -> T {
        match self {
            Self::Ref(value) => value.clone(),
            Self::Owned(value) => value,
        }
    }
}
