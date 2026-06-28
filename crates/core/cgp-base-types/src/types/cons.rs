/**
    The `Cons` type is used to represent the head of a _type-level list_,
    also known as an _anonymous product type_.

    `Cons` is used together with [`Nil`] to produce a type-level list using
    the `Product!` macro.

    ## Example

    Given the following product type definition:

    ```rust,ignore
    type MyTypes = Product![u32, String, bool];
    ```

    The following type would be generated:

    ```rust,ignore
    type MyTypes = Cons<u32, Cons<String, Cons<bool, Nil>>>;
    ```
*/
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Cons<Head, Tail>(pub Head, pub Tail);
