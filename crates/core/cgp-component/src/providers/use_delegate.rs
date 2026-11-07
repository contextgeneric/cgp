use core::marker::PhantomData;

/// The legacy provider that routes a component to a different inner provider per generic-parameter
/// type through a nested table; open replaces it.
///
/// See [`UseDelegate`](https://contextgeneric.dev/docs/reference/providers/use_delegate).
pub struct UseDelegate<Components>(pub PhantomData<Components>);
