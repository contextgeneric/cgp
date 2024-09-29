use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::traits::has_field::HasField;

pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Field;
}

impl<Context, Tag, Target, Field> HasFieldMut<Tag> for Context
where
    Context: DerefMut<Target = Target>,
    Target: HasFieldMut<Tag, Field = Field> + 'static,
{
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Field {
        self.deref_mut().get_field_mut(tag)
    }
}
