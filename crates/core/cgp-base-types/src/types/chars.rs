use core::fmt::Display;
use core::marker::PhantomData;

use crate::traits::StaticFormat;

/// The character list behind Symbol!, which spells a field name out one character at a time so the
/// name can be a type in trait resolution.
///
/// See [`Chars`](https://contextgeneric.dev/docs/reference/types/chars).
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct Chars<const CHAR: char, Tail>(pub PhantomData<Tail>);

impl<const CHAR: char, Tail> Display for Chars<CHAR, Tail>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}
