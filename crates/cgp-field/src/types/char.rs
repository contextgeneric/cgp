use core::fmt::Display;
use core::marker::PhantomData;

use crate::traits::StaticFormat;

/**
    The `Char` type, a.k.a. `ζ`, is used to represent _type-level_ list of
    `char`s, which are equivalent to type-level strings.

    `Char` is a specialized version of [`Cons`](crate::types::Cons), with the
    `Head` type being fixed to a _const-generic_ value of type `char`.
    Similar to `Cons`, `Char` is also parameterized by a `Tail` type, which is
    expected to be either the next `Char`, or [`Nil`](crate::types::Nil) to
    represent the end of the string.

    Instead of reusing `Cons`, we combine the use of `Cons` within `Char` so
    that its representation is more compact when shown in compiler error messages.
    Similar to `Cons`, `Char` is also shown as `ζ` to further improve its
    readability.

    We represent type-level strings as list of `Char`s, because it is currently
    not possible to use types like `String` or `&str` as const-generic parameters.
    On the other hand, a single `char` can be used as a const-generic parameter,
    and so we can workaround the limitation by combining a type-level list of
    `char`s to represent a type-level string.

    `Char` is most often not used directly, but rather through the `Symbol!` macro,
    which accepts a string literal and converts it into a list of `Char`s.

    ## Example

    Given the following symbol definition:

    ```rust,ignore
    type Hello = Symbol!("hello");
    ```

    The following type would be generated:

    ```rust,ignore
    type Hello = Char<'h', Char<'e', Char<'l', Char<'l', Char<'o', Nil>>>>>;
    ```

    which would be shown with the shortened representation as:

    ```rust,ignore
    type Hello = ζ<'h', ζ<'e', ζ<'l', ζ<'l', ζ<'o', ε>>>>>;
    ```
*/
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct ζ<const CHAR: char, Tail>(pub PhantomData<Tail>);

pub use ζ as Char;

impl<const CHAR: char, Tail> Display for Char<CHAR, Tail>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}
