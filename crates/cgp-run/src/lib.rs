#![no_std]

extern crate alloc;

#[allow(unused_imports)]
use alloc::boxed::Box;

use cgp_async::*;
use cgp_component::*;
use cgp_error::HasErrorType;

#[derive_component(RunnerComponent, Runner<Context>)]
#[async_trait]
pub trait CanRun: Async + HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}
