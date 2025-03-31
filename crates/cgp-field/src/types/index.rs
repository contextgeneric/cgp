use core::fmt::{Debug, Display};

#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct δ<const I: usize>;

pub use δ as Index;

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
