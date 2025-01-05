use cgp_core::error::ProvideErrorType;

use crate::types::Error;

pub struct UseBoxedStdError;

impl<Context> ProvideErrorType<Context> for UseBoxedStdError {
    type Error = Error;
}
