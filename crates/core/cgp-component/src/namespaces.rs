/// The namespace a component registers into by default.
///
/// See [`DefaultNamespace`](https://contextgeneric.dev/docs/reference/traits/namespace/default_namespace).
pub trait DefaultNamespace<Components> {
    /// The provider the namespace delegates to.
    type Delegate;
}

/// The per-type defaults a namespace holds for one type parameter.
///
/// See [`DefaultImpls1`](https://contextgeneric.dev/docs/reference/traits/namespace/default_impls1).
pub trait DefaultImpls1<T, Components> {
    /// The provider the default delegates to.
    type Delegate;
}

/// The per-type defaults a namespace holds for two type parameters.
///
/// See [`DefaultImpls2`](https://contextgeneric.dev/docs/reference/traits/namespace/default_impls2).
pub trait DefaultImpls2<T1, T2, Components> {
    /// The provider the default delegates to.
    type Delegate;
}
