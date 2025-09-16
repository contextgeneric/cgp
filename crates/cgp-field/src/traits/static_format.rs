use core::fmt::{self, Formatter};

use crate::types::{Char, Nil};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
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

impl StaticFormat for Nil {
    fn fmt(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
