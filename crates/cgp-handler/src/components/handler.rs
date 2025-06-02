use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component {
    provider: Handler,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
#[async_trait]
pub trait CanHandle<Code: Send, Input: Send>: HasAsyncErrorType {
    type Output: Send;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
