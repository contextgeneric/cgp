use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

pub trait HasField<Tag> {
    type Field;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Field;
}

pub trait FieldGetter<Context, Tag> {
    type Field;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Self::Field;
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

impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseContext
where
    Context: HasField<Tag, Field = Field>,
{
    type Field = Field;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Field {
        context.get_field(PhantomData)
    }
}
