use core::marker::PhantomData;

use cgp_component::WithProvider;
use cgp_type::traits::has_type::ProvideType;

use crate::traits::has_field::{FieldGetter, HasField};
use crate::traits::has_field_mut::{HasFieldMut, MutFieldGetter};

pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithField<Tag> = WithProvider<UseField<Tag>>;

impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Field = Field>,
{
    type Type = Field;
}

impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseField<Tag>
where
    Context: HasField<Tag, Field = Field>,
{
    type Field = Field;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Self::Field {
        context.get_field(tag)
    }
}

impl<Context, Tag, Field> MutFieldGetter<Context, Tag> for UseField<Tag>
where
    Context: HasFieldMut<Tag, Field = Field>,
{
    type Field = Field;

    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Field {
        context.get_field_mut(tag)
    }
}
