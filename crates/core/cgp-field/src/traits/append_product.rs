use crate::types::{Cons, Nil};

/// Appends one item to a type-level list.
///
/// See [`AppendProduct`](https://contextgeneric.dev/docs/reference/traits/type-level/append_product).
pub trait AppendProduct<Item: ?Sized> {
    /// The list with the item appended.
    type Output;
}

impl<Item> AppendProduct<Item> for Nil {
    type Output = Cons<Item, Nil>;
}

impl<Head, Tail, Item> AppendProduct<Item> for Cons<Head, Tail>
where
    Tail: AppendProduct<Item>,
{
    type Output = Cons<Head, Tail::Output>;
}
