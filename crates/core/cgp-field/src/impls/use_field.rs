use core::marker::PhantomData;

use cgp_component::{IsProviderFor, WithProvider};
use cgp_type::{TypeProvider, TypeProviderComponent};

use crate::traits::{FieldGetter, HasField, HasFieldMut, MutFieldGetter};

/// The provider that implements a getter by reading the context field a type-level tag names, so
/// the field name is a wiring choice, not the method name.
///
/// See [`UseField`](https://contextgeneric.dev/docs/reference/providers/use_field).
pub struct UseField<Tag>(pub PhantomData<Tag>);

/// The alias WithProvider`<UseField<Tag>`>: a single-method getter reads the field Tag names, and a
/// `#[cgp_type]` component takes that field's type.
///
/// See [`WithField`](https://contextgeneric.dev/docs/reference/providers/with_field).
pub type WithField<Tag> = WithProvider<UseField<Tag>>;

impl<Context, TypeTag, FieldTag, Field> TypeProvider<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Value = Field>,
{
    type Type = Field;
}

impl<Context, TypeTag, FieldTag, Field> IsProviderFor<TypeProviderComponent, Context, TypeTag>
    for UseField<FieldTag>
where
    Context: HasField<FieldTag, Value = Field>,
{
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
