use cgp_component::AppendProduct;

use crate::types::{Cons, Nil};

impl<Item> AppendProduct<Item> for Nil {
    type Output = Cons<Item, Nil>;
}

impl<Head, Tail, Item> AppendProduct<Item> for Cons<Head, Tail>
where
    Tail: AppendProduct<Item>,
{
    type Output = Cons<Head, Tail::Output>;
}
