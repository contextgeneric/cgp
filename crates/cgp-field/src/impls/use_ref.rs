use core::marker::PhantomData;

use cgp_component::WithProvider;

use crate::{FieldGetter, HasField, HasFieldMut, MutFieldGetter};

pub struct UseFieldRef<Tag, Value>(pub PhantomData<(Tag, Value)>);

pub type WithFieldRef<Tag, Value> = WithProvider<UseFieldRef<Tag, Value>>;

impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseFieldRef<Tag, Value>
where
    Context: HasField<Tag, Value: AsRef<Value> + 'static>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData).as_ref()
    }
}

impl<Context, OutTag, Tag, Value> MutFieldGetter<Context, OutTag> for UseFieldRef<Tag, Value>
where
    Context: HasFieldMut<Tag, Value: AsRef<Value> + AsMut<Value> + 'static>,
{
    fn get_field_mut(context: &mut Context, _tag: PhantomData<OutTag>) -> &mut Value {
        context.get_field_mut(PhantomData).as_mut()
    }
}
