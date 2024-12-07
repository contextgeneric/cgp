use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value;
}

pub trait FieldGetter<Context, Tag> {
    type Value;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Self::Value;
}

impl<Context, Tag, Target, Value> HasField<Tag> for Context
where
    Context: Deref<Target = Target>,
    Target: HasField<Tag, Value = Value> + 'static,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.deref().get_field(tag)
    }
}

impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseContext
where
    Context: HasField<Tag, Value = Field>,
{
    type Value = Field;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value {
        context.get_field(PhantomData)
    }
}
