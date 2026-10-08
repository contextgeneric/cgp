use crate::core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use crate::core::prelude::*;

/// Sets the context's abstract error type to [`Error`](crate::std_error::Error), a boxed standard error.
pub struct UseBoxedStdError;

#[cgp_impl(UseBoxedStdError)]
impl ErrorTypeProvider {
    type Error = crate::std_error::Error;
}
