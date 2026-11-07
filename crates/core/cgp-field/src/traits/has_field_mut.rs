use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::traits::{FieldGetter, HasField};

/// Mutably reads a field by its type-level tag.
///
/// See [`HasFieldMut`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field_mut).
pub trait HasFieldMut<Tag>: HasField<Tag> {
    /// Mutably reads the field.
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value;
}

/// Mutably reads a field from a context through a provider.
///
/// See [`MutFieldGetter`](https://contextgeneric.dev/docs/reference/traits/field-access/mut_field_getter).
pub trait MutFieldGetter<Context, Tag>: FieldGetter<Context, Tag> {
    /// Mutably reads the field.
    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Value;
}

#[diagnostic::do_not_recommend]
impl<Context, Tag, Target, Value> HasFieldMut<Tag> for Context
where
    Context: DerefMut<Target = Target>,
    Target: HasFieldMut<Tag, Value = Value> + 'static,
{
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value {
        self.deref_mut().get_field_mut(tag)
    }
}
