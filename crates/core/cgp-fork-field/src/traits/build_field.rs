use core::marker::PhantomData;

use crate::impls::{IsNothing, IsPresent};
use crate::traits::{PartialData, UpdateField};

pub trait BuildField<Tag> {
    type Value;

    type Output;

    fn build_field(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self::Output;
}

impl<Context, Tag> BuildField<Tag> for Context
where
    Context: UpdateField<Tag, IsPresent, Mapper = IsNothing>,
{
    type Value = Context::Value;

    type Output = Context::Output;

    fn build_field(self, tag: PhantomData<Tag>, value: Self::Value) -> Self::Output {
        self.update_field(tag, value).1
    }
}

pub trait FinalizeBuild: PartialData {
    fn finalize_build(self) -> Self::Target;
}
