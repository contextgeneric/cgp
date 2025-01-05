use core::fmt::Display;

pub struct Index<const I: usize>;

impl<const I: usize> Display for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{I}")
    }
}
