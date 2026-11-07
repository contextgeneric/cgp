use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

#[cgp_component(TryComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// A synchronous, fallible transform.
///
/// See [`CanTryCompute`](https://contextgeneric.dev/docs/reference/components/handler/try_computer).
pub trait CanTryCompute<Code, Input> {
    /// The transform's output.
    type Output;

    /// Computes the output from the input.
    fn try_compute(&self, _code: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

#[cgp_component(TryComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// A synchronous, fallible transform that borrows its input.
///
/// See [`CanTryComputeRef`](https://contextgeneric.dev/docs/reference/components/handler/try_computer_ref).
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
