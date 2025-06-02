use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component {
    provider: Computer,
    derive_delegate: UseDelegate<Code>,
}]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _tag: PhantomData<Code>, input: Input) -> Self::Output;
}
