use core::fmt::Display;
use core::marker::PhantomData;

pub struct ψ<const LEN: usize, Chars>(pub PhantomData<Chars>);

pub use ψ as Symbol;

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
