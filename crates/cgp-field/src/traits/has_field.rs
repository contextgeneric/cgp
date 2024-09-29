use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub trait HasField<Tag> {
    type Field;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Field;
}

pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Field;
}

impl<Context, Tag, Target, Field> HasField<Tag> for Context
where
    Context: Deref<Target = Target>,
    Target: HasField<Tag, Field = Field> + 'static,
{
    type Field = Field;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Field {
        self.deref().get_field(tag)
    }
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
