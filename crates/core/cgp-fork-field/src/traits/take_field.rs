use core::marker::PhantomData;

use crate::impls::{IsNothing, IsPresent};
use crate::traits::UpdateField;

pub trait TakeField<Tag> {
    type Value;

    type Remainder;

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
