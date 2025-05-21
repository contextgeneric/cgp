use core::fmt::Display;
use core::marker::PhantomData;

#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct ι<const CHAR: char, Tail>(pub PhantomData<Tail>);

pub use ι as Char;

use crate::StaticFormat;

impl<const CHAR: char, Tail> Display for Char<CHAR, Tail>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}

impl<const CHAR: char, Tail> StaticFormat for Char<CHAR, Tail>
where
    Tail: StaticFormat,
{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{CHAR}")?;
        Tail::fmt(f)
    }
}
