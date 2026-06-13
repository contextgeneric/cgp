use cgp::error::{ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;

use crate::types::Error;

pub struct UseBoxedStdError;

#[cgp_provider(ErrorTypeProviderComponent)]
impl<Context> ErrorTypeProvider<Context> for UseBoxedStdError {
    type Error = Error;
}
