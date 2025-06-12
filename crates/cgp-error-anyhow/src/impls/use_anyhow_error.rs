use anyhow::Error;
use cgp_core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_core::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorTypeProvider<Context> for UseAnyhowError {
    type Error = Error;
}
