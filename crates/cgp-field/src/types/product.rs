use crate::traits::StaticFormat;

/**
    The `Cons` type, a.k.a. `π`, is used to represent the head of a _type-level list_,
    also known as an _anonymous product type_.

    `Cons` is used together with [`Nil`] to produce a type-level list using
    the `Product!` macro.

    `Cons` is also shown as `π`, together with [`Nil`] shown as `ε`, to improve the
    readability of compiler error messages. Through the shortened name, a product
    type would take slightly less space, making it more likely to fit on a single
    line for the user to read what the type is.

    ## Example

    Given the following product type definition:

    ```rust,ignore
    type MyTypes = Product![u32, String, bool];
    ```

    The following type would be generated:

    ```rust,ignore
    type MyTypes = Cons<u32, Cons<String, Cons<bool, Nil>>>;
    ```

    which would be shown with the shortened representation as:

    ```rust,ignore
    type MyTypes = π<u32, π<String, π<bool, ε>>>;
    ```
*/
#[derive(Eq, PartialEq, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct π<Head, Tail>(pub Head, pub Tail);

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

pub use {ε as Nil, π as Cons};

impl StaticFormat for Nil {
    fn fmt(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
