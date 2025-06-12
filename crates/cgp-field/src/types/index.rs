use core::fmt::{Debug, Display};

/**
    The `Index` type, a.k.a. `δ`, is used to represent a `usize` value at
    the _type level_.

    `Index` is simply defined to be parameterized by a _const-generic_ value
    of type `usize`. It is most often used to access generic fields by their
    _index_, instead of by their _name_.

    `Index` is also shown as `δ` to improve the readability of compiler error
    messages.

    ## Example

    Given the following struct definition:

    ```rust,ignore
    pub struct MyContext(pub u32);
    ```

    The following `HasField` implementation would be generated, with use of
    `Index<0>` as the field tag:

    ```rust,ignore
    impl HasField<Index<0>> for MyContext {
        type Value = u32;

        fn get_field(&self) -> &u32 {
            &self.0
        }
    }
*/
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct δ<const I: usize>;

pub use δ as Index;

impl<const I: usize> Display for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{I}")
    }
}

impl<const I: usize> Debug for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{I}")
    }
}
