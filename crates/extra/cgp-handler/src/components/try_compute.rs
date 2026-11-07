use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

/// The handler-family component for a synchronous computation that can fail, returning a Result in
/// the context's abstract error type.
///
/// See [`CanTryCompute`](https://contextgeneric.dev/docs/reference/components/handler/try_computer)
/// .
#[cgp_component(TryComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
pub trait CanTryCompute<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the input.
    fn try_compute(&self, _code: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

/// The handler-family component for a synchronous computation that can fail and only reads its
/// input, returning a Result in the context's error type.
///
/// See
/// [`CanTryComputeRef`](https://contextgeneric.dev/docs/reference/components/handler/try_computer_ref)
/// .
#[cgp_component(TryComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
pub trait CanTryComputeRef<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the borrowed input.
    fn try_compute_ref(
        &self,
        _code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Error>;
}
