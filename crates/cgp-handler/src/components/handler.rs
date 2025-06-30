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
pub trait CanHandle<Code, Input>: HasAsyncErrorType {
    type Output;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

#[async_trait]
#[cgp_component {
    provider: HandlerRef,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanHandleRef<Code, Input>: HasAsyncErrorType {
    type Output;

    async fn handle_ref(
        &self,
        _tag: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Self::Error>;
}
