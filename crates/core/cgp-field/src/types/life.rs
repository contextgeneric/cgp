use core::marker::PhantomData;

/// A lifetime lifted into a type, so a component's lifetime parameter can travel through the
/// dependency marker, whose parameters are types.
///
/// See [`Life`](https://contextgeneric.dev/docs/reference/types/life).
pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
