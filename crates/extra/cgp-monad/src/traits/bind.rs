/// Binds a provider into a monad.
///
/// See [`MonadicBind`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_bind).
pub trait MonadicBind<Provider> {
    /// The bound provider.
    type Provider;
}
