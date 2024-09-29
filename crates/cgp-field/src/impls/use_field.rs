use core::marker::PhantomData;

use cgp_component::WithProvider;
use cgp_type::traits::has_type::ProvideType;

use crate::HasField;

pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithFieldType<Tag> = WithProvider<UseField<Tag>>;

impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Field = Field>,
{
    type Type = Field;
}
