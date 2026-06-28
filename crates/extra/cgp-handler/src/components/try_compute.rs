use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

use crate::UseInputDelegate;

#[cgp_component(TryComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanTryCompute<Code, Input>: HasErrorType {
    type Output;

    fn try_compute(
        &self,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

#[cgp_component(TryComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanTryComputeRef<Code, Input>: HasErrorType {
    type Output;

    fn try_compute_ref(
        &self,
        _code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Self::Error>;
}
