use core::marker::PhantomData;

use crate::traits::{FieldGetter, HasField};

#[diagnostic::on_unimplemented(
    message = "HasFieldMut is not implemented for {Self} with the field: {Tag}",
    note = "You need to add #[derive(HasField)] to {Self} with the given field present in the struct"
)]
pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value;
}

pub trait MutFieldGetter<Context, Tag>: FieldGetter<Context, Tag> {
    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Value;
}

impl<'a, Context, Tag, Value> HasFieldMut<Tag> for &'a mut Context
where
    Context: HasFieldMut<Tag, Value = Value>,
{
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value {
        Context::get_field_mut(self, tag)
    }
}
