use core::marker::PhantomData;

/// A type-level path segment.
///
/// See [`PathCons`](https://contextgeneric.dev/docs/reference/types/path_cons).
pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);
