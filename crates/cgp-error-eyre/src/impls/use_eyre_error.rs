use cgp_core::error::ErrorTypeProvider;
use eyre::Error;

pub struct UseEyreError;

impl<Context> ErrorTypeProvider<Context> for UseEyreError {
    type Error = Error;
}
