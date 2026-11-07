/// Borrows the value inside an `Option` as a `&T`.
///
/// The blanket impl covers any `Option<U>` whose `U` implements `AsRef<T>`, so an
/// `#[implicit] nickname: Option<&str>` argument reads an `Option<String>`, an
/// `Option<Cow<str>>`, or an `Option<Box<str>>` field without needing an allocator
/// of its own.
pub trait OptionAsRef<T: ?Sized> {
    /// Borrows the contained value, or `None` when the option is empty.
    fn option_as_ref(&self) -> Option<&T>;
}

impl<T: ?Sized, U> OptionAsRef<T> for Option<U>
where
    U: AsRef<T>,
{
    fn option_as_ref(&self) -> Option<&T> {
        self.as_ref().map(|value| value.as_ref())
    }
}
