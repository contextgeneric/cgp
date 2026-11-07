use core::fmt::Display;
use core::marker::PhantomData;

/// A type-level string, used as the tag that names a struct field in CGP's field-access traits.
///
/// See [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol).
pub struct Symbol<const LEN: usize, Chars>(pub PhantomData<Chars>);

use crate::traits::StaticFormat;

impl<const LEN: usize, Chars> Default for Symbol<LEN, Chars> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<const LEN: usize, Chars> StaticFormat for Symbol<LEN, Chars>
where
    Chars: StaticFormat,
{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Chars::fmt(f)
    }
}

impl<const LEN: usize, Chars> Display for Symbol<LEN, Chars>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}
