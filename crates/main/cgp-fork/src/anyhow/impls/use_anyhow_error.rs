use crate::core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use crate::core::prelude::*;

/// Sets the context's abstract error type to [`anyhow::Error`].
pub struct UseAnyhowError;

#[cgp_impl(UseAnyhowError)]
impl ErrorTypeProvider {
    type Error = anyhow::Error;
}
