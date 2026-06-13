use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;
use eyre::Error;

#[cgp_new_provider]
impl<Context> ErrorTypeProvider<Context> for UseEyreError {
    type Error = Error;
}
