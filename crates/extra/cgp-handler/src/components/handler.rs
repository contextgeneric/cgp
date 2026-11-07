use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

#[async_trait]
#[cgp_component(Handler)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// An asynchronous, fallible, error-aware computation.
///
/// See [`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler).
pub trait CanHandle<Code, Input> {
    /// The handler's output.
    type Output;

    /// Handles the input.
    async fn handle(&self, _tag: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

#[async_trait]
#[cgp_component(HandlerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// An asynchronous, fallible, error-aware computation that borrows its input.
///
/// See [`CanHandleRef`](https://contextgeneric.dev/docs/reference/components/handler/handler_ref).
pub trait CanHandleRef<Code, Input> {
    /// The handler's output.
    type Output;

    /// Handles the borrowed input.
    async fn handle_ref(
        &self,
        _tag: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Error>;
}
