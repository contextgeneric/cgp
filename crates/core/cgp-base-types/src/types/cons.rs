/// The head-and-tail cell of the type-level product list, which describes a record one field at a
/// time so generic code can walk any struct's fields.
///
/// See [`Cons`](https://contextgeneric.dev/docs/reference/types/cons).
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Cons<Head, Tail>(pub Head, pub Tail);
