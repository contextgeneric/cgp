use crate::StaticFormat;

#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct π<Head, Tail>(pub Head, pub Tail);

#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct ε;

pub use {ε as Nil, π as Cons};

impl StaticFormat for Nil {
    fn fmt(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
