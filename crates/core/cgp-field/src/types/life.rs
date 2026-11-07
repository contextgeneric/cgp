use core::marker::PhantomData;

/// A lifetime lifted into a type.
///
/// See [`Life`](https://contextgeneric.dev/docs/reference/types/life).
pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
