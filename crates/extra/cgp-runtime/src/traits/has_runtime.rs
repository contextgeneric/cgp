use cgp::prelude::*;

use crate::HasRuntimeType;

/// The getter component that borrows the runtime value a context stores, so effectful generic code
/// reaches Tokio, a mock, or a test executor generically.
///
/// See [`HasRuntime`](https://contextgeneric.dev/docs/reference/components/has_runtime).
#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
pub trait HasRuntime {
    /// Returns the context's runtime.
    fn runtime(&self) -> &Runtime;
}
