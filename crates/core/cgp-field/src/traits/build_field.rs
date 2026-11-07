use core::marker::PhantomData;

use crate::impls::{IsNothing, IsPresent};
use crate::traits::{PartialData, UpdateField};

/// Set a currently-absent field on a partial record, which is the direction of the builder family
/// you write most when assembling a struct field by field.
///
/// See [`BuildField`](https://contextgeneric.dev/docs/reference/traits/builder/build_field).
pub trait BuildField<Tag> {
    /// The field's value type.
    type Value;

    /// The record with the field built.
    type Output;

    /// Builds the field.
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

/// Turn a partial record with every field present back into the concrete struct; the impl exists
/// only at that configuration.
///
/// See [`FinalizeBuild`](https://contextgeneric.dev/docs/reference/traits/builder/finalize_build).
pub trait FinalizeBuild: PartialData {
    /// Finalizes the builder.
    fn finalize_build(self) -> Self::Target;
}
