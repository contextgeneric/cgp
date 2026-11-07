/// The monad trait applying one monad as a transformer over another, so a pipeline can peel a
/// nested Result; PipeMonadic uses it for fallible steps.
///
/// See [`MonadicTrans`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_trans).
pub trait MonadicTrans<M> {
    /// The transformed monad.
    type M;
}
