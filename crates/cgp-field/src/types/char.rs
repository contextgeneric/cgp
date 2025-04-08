use core::fmt::Display;

#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct ι<const CHAR: char, Tail>(pub Tail);

pub use ι as Char;

impl<const CHAR: char, Tail> Display for Char<CHAR, Tail>
where
    Tail: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{CHAR}")?;
        self.0.fmt(f)
    }
}
