/**
    The `Nil` type, a.k.a. `ε`, is used to represent the end of a _type-level list_,
    or an empty type-level list.

    `Nil` is commonly used as the `Tail` of a [`Cons`] type, to terminate the list.
    When used on its own, it represents an empty type-level list.

    Read more about type-level lists, a.k.a. the product types, in [`Cons`].
*/
#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct ε;

pub use ε as Nil;
