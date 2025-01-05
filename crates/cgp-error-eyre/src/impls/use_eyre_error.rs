use cgp_core::error::ProvideErrorType;
use eyre::Error;

pub struct UseEyreError;

impl<Context> ProvideErrorType<Context> for UseEyreError {
    type Error = Error;
}
