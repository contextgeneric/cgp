/// Borrow the value inside an Option as a &T, the bound an Option`<&str>` implicit argument
/// generates so the field may be a String, a Cow`<str>`, or a Box`<str>`.
///
/// The blanket impl covers any `Option<U>` whose `U` implements `AsRef<T>`, so an
/// `#[implicit] nickname: Option<&str>` argument reads an `Option<String>` field
/// without needing an allocator.
///
/// See [`OptionAsRef`](https://contextgeneric.dev/docs/reference/traits/field-access/option_as_ref)
/// .
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
