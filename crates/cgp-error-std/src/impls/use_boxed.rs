use cgp_core::error::ErrorTypeProvider;

use crate::types::Error;

pub struct UseBoxedStdError;

impl<Context> ErrorTypeProvider<Context> for UseBoxedStdError {
    type Error = Error;
}
