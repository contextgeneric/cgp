use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

use crate::UseInputDelegate;

#[async_trait]
#[cgp_component {
    provider: Handler,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanHandle<Code: Send, Input: Send>: HasAsyncErrorType {
    type Output: Send;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}
