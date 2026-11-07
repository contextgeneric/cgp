use core::marker::PhantomData;

/// The cell of a type-level path, the list of segments that names a route through nested delegation
/// tables so a lookup can reach an entry behind a prefix.
///
/// See [`PathCons`](https://contextgeneric.dev/docs/reference/types/path_cons).
pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);
