use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

/// The trait for reading one field of a context by its name as a type, so generic code can bound on
/// a field of a type it cannot name.
///
/// See [`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field).
pub trait HasField<Tag> {
    /// The field's value type.
    type Value;

    /// Reads the field.
    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}

/// The provider-side form of HasField: a getter component takes any FieldGetter provider, such as
/// UseField, through WithProvider.
///
/// See [`FieldGetter`](https://contextgeneric.dev/docs/reference/traits/field-access/field_getter).
pub trait FieldGetter<Context, Tag> {
    /// The field's value type.
    type Value;

    /// Reads the field.
    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value;
}

#[diagnostic::do_not_recommend]
impl<Context, Tag, Target, Value> HasField<Tag> for Context
where
    Context: DerefMap<Target = Target>,
    Target: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.map_deref(|context| context.get_field(tag))
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

/**
   A helper trait to help organize the lifetime inference in Rust.
   Without this, `Self::Target` would need to be `'static`, as Rust couldn't
   infer the correct lifetime when calling `context.deref().get_field()`.
*/
trait DerefMap: Deref {
    fn map_deref<T>(&self, mapper: impl for<'a> FnOnce(&'a Self::Target) -> &'a T) -> &T;
}

impl<Context> DerefMap for Context
where
    Context: Deref,
{
    fn map_deref<T>(&self, mapper: impl for<'a> FnOnce(&'a Self::Target) -> &'a T) -> &T {
        mapper(self.deref())
    }
}
