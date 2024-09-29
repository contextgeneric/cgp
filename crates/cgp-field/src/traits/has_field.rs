use core::marker::PhantomData;
use core::ops::Deref;

pub trait HasField<Tag> {
    type Field;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Field;
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
