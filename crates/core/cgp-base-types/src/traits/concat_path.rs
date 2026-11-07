use crate::types::{Nil, PathCons};

/// Join two type-level paths, as the generated lookup behind namespaces and open does to append a
/// component's type parameters to its route.
///
/// See [`ConcatPath`](https://contextgeneric.dev/docs/reference/traits/formatting/concat_path).
pub trait ConcatPath<Other: ?Sized> {
    /// The concatenated path.
    type Output: ?Sized;
}

impl<Head: ?Sized, Tail: ?Sized, Other: ?Sized> ConcatPath<Other> for PathCons<Head, Tail>
where
    Tail: ConcatPath<Other>,
{
    type Output = PathCons<Head, <Tail as ConcatPath<Other>>::Output>;
}

impl<Other: ?Sized> ConcatPath<Other> for Nil {
    type Output = Other;
}
