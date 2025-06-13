use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

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

pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
