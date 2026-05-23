use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
    provider: Producer,
    derive_delegate: UseDelegate<Code>,
}]
pub trait CanProduce<Code> {
    type Output;

    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
