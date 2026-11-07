/// The namespace trait a context joins with a namespace header, resolving each component's default
/// route; prefix registers components into it.
///
/// See
/// [`DefaultNamespace`](https://contextgeneric.dev/docs/reference/traits/namespace/default_namespace)
/// .
pub trait DefaultNamespace<Components> {
    /// The provider the namespace delegates to.
    type Delegate;
}

/// The lookup trait for a namespace default keyed on a component and one type, which default_impl
/// registers into and a for loop reads back.
///
/// See [`DefaultImpls1`](https://contextgeneric.dev/docs/reference/traits/namespace/default_impls1)
/// .
pub trait DefaultImpls1<T, Components> {
    /// The provider the default delegates to.
    type Delegate;
}

/// The lookup trait for a namespace default keyed on a component and a pair of types, reachable
/// through default_impl and a for loop.
///
/// See [`DefaultImpls2`](https://contextgeneric.dev/docs/reference/traits/namespace/default_impls2)
/// .
pub trait DefaultImpls2<T1, T2, Components> {
    /// The provider the default delegates to.
    type Delegate;
}
