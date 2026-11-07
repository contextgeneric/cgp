use core::marker::PhantomData;

use crate::impls::{IsNothing, IsPresent};
use crate::traits::UpdateField;

/// Takes one field out of a value.
///
/// See [`TakeField`](https://contextgeneric.dev/docs/reference/traits/builder/take_field).
pub trait TakeField<Tag> {
    /// The field's value type.
    type Value;

    /// The value left after the field is taken.
    type Remainder;

    /// Takes the field.
    fn take_field(self, _tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder);
}

impl<Context, Tag> TakeField<Tag> for Context
where
    Context: UpdateField<Tag, IsNothing, Mapper = IsPresent>,
{
    type Value = Context::Value;

    type Remainder = Context::Output;

    fn take_field(self, tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder) {
        self.update_field(tag, ())
    }
}
