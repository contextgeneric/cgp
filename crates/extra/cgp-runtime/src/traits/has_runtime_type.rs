use cgp::prelude::*;

/// The abstract-type component that declares the runtime type a context runs against, chosen by
/// wiring, for code that names the type but not a value.
///
/// See [`HasRuntimeType`](https://contextgeneric.dev/docs/reference/components/has_runtime_type).
#[cgp_type]
pub trait HasRuntimeType {
    /// The context's runtime type.
    type Runtime;
}

/// The runtime type a context chooses.
pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
