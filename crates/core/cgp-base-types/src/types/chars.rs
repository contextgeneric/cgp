use core::fmt::Display;
use core::marker::PhantomData;

use crate::traits::StaticFormat;

/**
    The `Chars` type is used to represent _type-level_ list of
    `Chars`s, which are equivalent to type-level strings.

    `Chars` is a specialized version of [`Cons`](crate::types::Cons), with the
    `Head` type being fixed to a _const-generic_ value of type `Chars`.
    Similar to `Cons`, `Chars` is also parameterized by a `Tail` type, which is
    expected to be either the next `Chars`, or [`Nil`](crate::types::Nil) to
    represent the end of the string.

    We represent type-level strings as list of `Chars`s, because it is currently
    not possible to use types like `String` or `&str` as const-generic parameters.
    On the other hand, a single `Chars` can be used as a const-generic parameter,
    and so we can workaround the limitation by combining a type-level list of
    `Chars`s to represent a type-level string.

    `Chars` is most often not used directly, but rather through the `Symbol!` macro,
    which accepts a string literal and converts it into a list of `Chars`s.

    ## Example

    Given the following symbol definition:

    ```rust,ignore
    type Hello = Symbol!("hello");
    ```

    The following type would be generated:

    ```rust,ignore
    type Hello = Chars<'h', Chars<'e', Chars<'l', Chars<'l', Chars<'o', Nil>>>>>;
    ```
*/
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct Chars<const CHAR: char, Tail>(pub PhantomData<Tail>);

impl<const CHAR: char, Tail> Display for Chars<CHAR, Tail>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}
