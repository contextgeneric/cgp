use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;

/// Sets the context's abstract error type to [`anyhow::Error`].
pub struct UseAnyhowError;

#[cgp_impl(UseAnyhowError)]
impl ErrorTypeProvider {
    type Error = anyhow::Error;
}
