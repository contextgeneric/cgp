use core::marker::PhantomData;

/// Redirects a component lookup along a type-level path.
///
/// See [`RedirectLookup`](https://contextgeneric.dev/docs/reference/providers/redirect_lookup).
pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);
