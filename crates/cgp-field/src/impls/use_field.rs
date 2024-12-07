use core::marker::PhantomData;

use cgp_component::WithProvider;
use cgp_type::traits::has_type::ProvideType;

use crate::traits::has_field::{FieldGetter, HasField};
use crate::traits::has_field_mut::{HasFieldMut, MutFieldGetter};

pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithField<Tag> = WithProvider<UseField<Tag>>;

impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Value = Field>,
{
    type Type = Field;
}

impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData)
    }
}

impl<Context, OutTag, Tag, Value> MutFieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasFieldMut<Tag, Value = Value>,
{
    fn get_field_mut(context: &mut Context, _tag: PhantomData<OutTag>) -> &mut Value {
        context.get_field_mut(PhantomData)
    }
}
