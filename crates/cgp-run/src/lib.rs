#![no_std]

use core::future::Future;
use core::marker::PhantomData;

use cgp_core::prelude::*;

#[cgp_component {
    provider: Runner,
    derive_delegate: UseDelegate<Code>,
}]
#[async_trait]
pub trait CanRun<Code>: HasErrorType {
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Self::Error>;
}

#[cgp_component {
    provider: SendRunner,
    derive_delegate: UseDelegate<Code>,
}]
#[async_trait]
pub trait CanSendRun<Code>: HasErrorType {
    fn send_run(
        &self,
        _code: PhantomData<Code>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
