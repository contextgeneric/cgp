use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;

/// Sets the context's abstract error type to [`eyre::Report`].
pub struct UseEyreError;

#[cgp_impl(UseEyreError)]
impl ErrorTypeProvider {
    type Error = eyre::Report;
}
