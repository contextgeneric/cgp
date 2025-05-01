#![no_std]

use cgp_async::*;
use cgp_component::*;
use cgp_error::HasAsyncErrorType;
use cgp_macro::*;

#[cgp_component {
    provider: Runner,
}]
#[async_trait]
pub trait CanRun: HasAsyncErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}
