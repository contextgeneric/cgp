use cgp::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
/// A getter for the context's async runtime.
///
/// See [`HasRuntime`](https://contextgeneric.dev/docs/reference/components/has_runtime).
pub trait HasRuntime {
    /// Returns the context's runtime.
    fn runtime(&self) -> &Runtime;
}
