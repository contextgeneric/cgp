use cgp_core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_core::prelude::*;
use eyre::Error;

pub struct UseEyreError;

#[cgp_provider(ErrorTypeProviderComponent)]
impl<Context> ErrorTypeProvider<Context> for UseEyreError {
    type Error = Error;
}
