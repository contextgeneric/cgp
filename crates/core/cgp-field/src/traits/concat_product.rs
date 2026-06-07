use crate::types::{Cons, Nil};

pub trait ConcatProduct<Items> {
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
