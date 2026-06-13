use anyhow::Error;
use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context> ErrorTypeProvider<Context> for UseAnyhowError {
    type Error = Error;
}
