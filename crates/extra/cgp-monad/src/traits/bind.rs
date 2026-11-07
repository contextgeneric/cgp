/// The monad trait that turns a continuation provider into the bind step running it, BindErr for
/// ErrMonadic and BindOk for OkMonadic.
///
/// See [`MonadicBind`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_bind).
pub trait MonadicBind<Provider> {
    /// The bound provider.
    type Provider;
}
