use cgp_fork::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_fork::prelude::*;

/// Sets the context's abstract error type to [`anyhow::Error`].
pub struct UseAnyhowError;

#[cgp_impl(UseAnyhowError)]
impl ErrorTypeProvider {
    type Error = anyhow::Error;
}
