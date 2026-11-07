use core::marker::PhantomData;

/// The generated provider that answers a component by looking a type-level path up in a table, the
/// mechanism behind namespaces and the open statement.
///
/// See [`RedirectLookup`](https://contextgeneric.dev/docs/reference/providers/redirect_lookup).
pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);
