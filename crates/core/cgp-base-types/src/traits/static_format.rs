use core::fmt::{self, Formatter};

use crate::types::{Chars, Nil};

/// Formats a type-level string.
///
/// See [`StaticFormat`](https://contextgeneric.dev/docs/reference/traits/formatting/static_format).
pub trait StaticFormat {
    /// Writes the string to the formatter.
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

impl<const CHAR: char, Tail> StaticFormat for Chars<CHAR, Tail>
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
