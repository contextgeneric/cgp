use core::marker::PhantomData;

/// The adapter that turns a TypeProvider or FieldGetter into a named component's provider, spelled
/// in wiring through the With… aliases.
///
/// See [`WithProvider`](https://contextgeneric.dev/docs/reference/providers/with_provider).
pub struct WithProvider<Provider>(pub PhantomData<Provider>);
