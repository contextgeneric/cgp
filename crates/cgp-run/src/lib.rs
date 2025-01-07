#![no_std]

use cgp_async::*;
use cgp_component::*;
use cgp_component_macro::*;
use cgp_error::HasErrorType;

#[cgp_component {
    provider: Runner,
}]
#[async_trait]
pub trait CanRun: Async + HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}
