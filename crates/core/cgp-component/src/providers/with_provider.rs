use core::marker::PhantomData;

/// A provider that wraps another provider.
///
/// See [`WithProvider`](https://contextgeneric.dev/docs/reference/providers/with_provider).
pub struct WithProvider<Provider>(pub PhantomData<Provider>);
