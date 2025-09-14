use core::fmt::{self, Display, Formatter};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

pub trait StaticDisplay {
    const VALUE: &'static dyn Display;
}
