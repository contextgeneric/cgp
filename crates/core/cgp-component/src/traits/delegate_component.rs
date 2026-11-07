/// The trait a wiring table is made of: one impl per entry, mapping a key such as a component name
/// to the provider a context chose for it.
///
/// See
/// [`DelegateComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/delegate_component)
/// .
pub trait DelegateComponent<Key: ?Sized> {
    /// The provider the context delegates the component to.
    type Delegate;
}
