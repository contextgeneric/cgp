use anyhow::Error;
use cgp_core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_core::prelude::*;

pub struct UseAnyhowError;

#[cgp_provider(ErrorTypeProviderComponent)]
impl<Context> ErrorTypeProvider<Context> for UseAnyhowError {
    type Error = Error;
}
