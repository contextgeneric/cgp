use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

use crate::UseInputDelegate;

#[cgp_component {
    provider: TryComputer,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanTryCompute<Code, Input>: HasErrorType {
    type Output;

    fn try_compute(
        &self,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}

#[cgp_component {
    provider: TryComputerRef,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanTryComputeRef<Code, Input>: HasErrorType {
    type Output;

    fn try_compute_ref(
        &self,
        _code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Self::Error>;
}
