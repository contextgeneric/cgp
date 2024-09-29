use core::marker::PhantomData;

use cgp_type::traits::has_type::ProvideType;
use cgp_type::types::with_type::WithTypeProvider;

use crate::HasField;

pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithFieldType<Tag> = WithTypeProvider<UseField<Tag>>;

impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Field = Field>,
{
    type Type = Field;
}
