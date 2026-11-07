use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

/// The general handler-family component: an async computation that can fail, returning a Result in
/// the context's error type. Simpler members promote into it.
///
/// See [`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler).
#[async_trait]
#[cgp_component(Handler)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
pub trait CanHandle<Code, Input> {
    /// The handler's output.
    type Output;

    /// Handles the input.
    async fn handle(&self, _tag: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

/// The handler-family component for an async computation that can fail and only reads its input,
/// which its async method takes as &Input.
///
/// See [`CanHandleRef`](https://contextgeneric.dev/docs/reference/components/handler/handler_ref).
#[async_trait]
#[cgp_component(HandlerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
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
