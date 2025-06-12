/**
    The `Either` type, a.k.a. `σ`, is used to represent an _anonymous sum type_.

    Similar to [`Cons`](crate::types::Cons), `Either` is used to form a sum type
    by combining a chain of `Either` types, and terminated with a [`Void`] type.
    But unlike product types, a sum type has values that belong to one
    of the variants in the list.

    `Either` is also shown as `σ`, together with [`Void`] shown as `θ`, to improve
    the readability of compiler error messages. Through the shortened name, a sum
    type would take slightly less space, making it more likely to fit on a single
    line for the user to read what the type is.

    `Either` is most often used through the `Sum!` macro, which accepts a list of
    types and turns them into a chain of `Either` types.

    ## Example

    Given the following sum type definition:

    ```rust,ignore
    type MyUnion = Sum![u32, String, bool];
    ```

    The following type would be generated:

    ```rust,ignore
    type MyUnion = Either<u32, Either<String, Either<bool, Void>>>;
    ```

    which would be shown with the shortened representation as:

    ```rust,ignore
    type MyUnion = σ<u32, σ<String, σ<bool, θ>>>;
    ```
*/
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum σ<Head, Tail> {
    Left(Head),
    Right(Tail),
}

/**
    The `Void` type, a.k.a. `θ`, is used to represent the end of an _anonymous sum type_,
    or an _empty_ sum type.

    `Void` is commonly used as the `Tail` of a [`Either`] type, to terminate the list.
    When used on its own, it represents an empty sum type, which can _never be constructed_.

    `Void` is functionally the same as the
    [_never_ type](https://doc.rust-lang.org/std/primitive.never.html), `!`,
    or otherwise
    [`Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html).
    However, we define a separate `Void` type, to make it more clear that it is
    specifically used for terminating a sum type.

    Read more about sum types in [`Either`].
*/
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum θ {}

pub use {θ as Void, σ as Either};
