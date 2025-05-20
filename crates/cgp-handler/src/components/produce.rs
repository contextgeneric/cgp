use core::marker::PhantomData;

use cgp_core::prelude::*;

#[cgp_component(Producer)]
pub trait CanProduce<Tag> {
    type Output;

    fn produce(&self, _tag: PhantomData<Tag>) -> Self::Output;
}
