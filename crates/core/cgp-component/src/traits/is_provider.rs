/// The empty marker every provider trait requires, implemented under the provider's own where
/// clause so a check can name the dependency a provider is missing.
///
/// See [`IsProviderFor`](https://contextgeneric.dev/docs/reference/traits/wiring/is_provider_for).
pub trait IsProviderFor<Component, Context, Params: ?Sized = ()> {}
