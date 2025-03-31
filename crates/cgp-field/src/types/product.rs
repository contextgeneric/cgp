
#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct π<Head, Tail>(pub Head, pub Tail);

#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct ε;

pub type Cons<Head, Tail> = π<Head, Tail>;

pub type Nil = ε;

pub const fn cons<Head, Tail>(head: Head, tail: Tail) -> π<Head, Tail> {
    π(head, tail)
}

pub const fn nil() -> ε {
    ε
}