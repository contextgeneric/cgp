use cgp_core::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp_core::prelude::*;

use crate::types::Error;

pub struct UseBoxedStdError;

#[cgp_provider(ErrorTypeProviderComponent)]
impl<Context> ErrorTypeProvider<Context> for UseBoxedStdError {
    type Error = Error;
}
