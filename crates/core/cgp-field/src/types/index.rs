use core::fmt::{Debug, Display};

/// A number lifted into a type, so a tuple-struct field can be named by its position the way
/// Symbol! names a field by its string.
///
/// See [`Index`](https://contextgeneric.dev/docs/reference/types/index_type).
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct Index<const I: usize>;

impl<const I: usize> Display for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{I}")
    }
}

impl<const I: usize> Debug for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{I}")
    }
}
