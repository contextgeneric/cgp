use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

use crate::UseInputDelegate;

#[cgp_component {
    provider: Computer,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_component {
    provider: ComputerRef,
    derive_delegate: [
        UseDelegate<Code>,
        UseInputDelegate<Input>,
    ],
}]
pub trait CanComputeRef<Code, Input> {
    type Output;

    fn compute_ref(&self, _code: PhantomData<Code>, input: &Input) -> Self::Output;
}
