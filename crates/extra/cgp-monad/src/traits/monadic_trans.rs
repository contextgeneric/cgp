/// Transforms one monad into another.
///
/// See [`MonadicTrans`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_trans).
pub trait MonadicTrans<M> {
    /// The transformed monad.
    type M;
}
