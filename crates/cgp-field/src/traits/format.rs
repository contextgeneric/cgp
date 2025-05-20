use core::fmt::{self, Formatter};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}
