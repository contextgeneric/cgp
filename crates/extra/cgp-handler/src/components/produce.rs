use core::marker::PhantomData;

use cgp::component::UseDelegate;
use cgp::prelude::*;

/// The input-free handler-family component: a synchronous, infallible provider that produces an
/// Output from the context and a Code tag alone.
///
/// See [`CanProduce`](https://contextgeneric.dev/docs/reference/components/handler/producer).
#[cgp_component(Producer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanProduce<Code> {
    /// The produced output.
    type Output;

    /// Produces the output.
    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
