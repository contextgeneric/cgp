use cgp_core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_core::prelude::*;
use eyre::Error;

#[cgp_new_provider]
impl<Context> ErrorTypeProvider<Context> for UseEyreError {
    type Error = Error;
}
