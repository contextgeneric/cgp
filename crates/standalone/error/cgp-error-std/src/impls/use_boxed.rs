use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;

/// Sets the context's abstract error type to [`Error`](crate::Error), a boxed standard error.
pub struct UseBoxedStdError;

#[cgp_impl(UseBoxedStdError)]
impl ErrorTypeProvider {
    type Error = crate::Error;
}
