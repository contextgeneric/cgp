use crate::types::{Cons, Nil};

/// Concatenates two type-level lists.
///
/// See [`ConcatProduct`](https://contextgeneric.dev/docs/reference/traits/type-level/concat_product).
pub trait ConcatProduct<Items> {
    /// The concatenated list.
    type Output;
}

impl<Items> ConcatProduct<Items> for Nil {
    type Output = Items;
}

impl<Head, Tail, Items> ConcatProduct<Items> for Cons<Head, Tail>
where
    Tail: ConcatProduct<Items>,
{
    type Output = Cons<Head, Tail::Output>;
}
