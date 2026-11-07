use crate::types::{Cons, Nil};

/// Name the type-level product list formed by one product's entries followed by another's, the
/// general form of appending a single entry.
///
/// See
/// [`ConcatProduct`](https://contextgeneric.dev/docs/reference/traits/type-level/concat_product) .
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
