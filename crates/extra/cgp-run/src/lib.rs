#![no_std]

use core::future::Future;
use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanRun<Code>: HasErrorType {
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Self::Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanSendRun<Code>: HasErrorType {
    fn send_run(
        &self,
        _code: PhantomData<Code>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
