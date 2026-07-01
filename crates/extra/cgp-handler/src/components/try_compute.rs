use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

#[cgp_component(TryComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType::Error)]
pub trait CanTryCompute<Code, Input> {
    type Output;

    fn try_compute(&self, _code: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

#[cgp_component(TryComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType::Error)]
pub trait CanTryComputeRef<Code, Input> {
    type Output;

    fn try_compute_ref(
        &self,
        _code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Error>;
}
