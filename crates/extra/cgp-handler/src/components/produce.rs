use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component(Producer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
/// An input-free production.
///
/// See [`CanProduce`](https://contextgeneric.dev/docs/reference/components/handler/producer).
pub trait CanProduce<Code> {
    /// The produced output.
    type Output;

    /// Produces the output.
    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
