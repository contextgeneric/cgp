use cgp::prelude::*;

#[cgp_type]
/// A type the context chooses as its async runtime.
///
/// See [`HasRuntimeType`](https://contextgeneric.dev/docs/reference/components/has_runtime_type).
pub trait HasRuntimeType {
    /// The context's runtime type.
    type Runtime;
}

/// The runtime type a context chooses.
pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
