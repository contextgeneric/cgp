#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct π<Head, Tail>(pub Head, pub Tail);

#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct ε;

pub use {ε as Nil, π as Cons};
