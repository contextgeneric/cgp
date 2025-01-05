use anyhow::Error;
use cgp_core::error::ProvideErrorType;

pub struct UseAnyhowError;

impl<Context> ProvideErrorType<Context> for UseAnyhowError {
    type Error = Error;
}
