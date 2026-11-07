use core::marker::PhantomData;

use crate::traits::{FieldGetter, HasField};

/// Maps a field's value through a closure.
///
/// See [`MapField`](https://contextgeneric.dev/docs/reference/traits/field-access/map_field).
pub trait MapField<Tag>: HasField<Tag> {
    /// Maps the field's value.
    fn map_field<T>(
        &self,
        _tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T;
}

impl<Context, Tag> MapField<Tag> for Context
where
    Context: HasField<Tag>,
    Tag: 'static,
{
    fn map_field<T>(
        &self,
        tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T {
        mapper(self.get_field(tag))
    }
}

/// The provider trait equivalent of [`MapField`].
///
/// See [`FieldMapper`](https://contextgeneric.dev/docs/reference/traits/field-access/field_mapper).
pub trait FieldMapper<Context, Tag>: FieldGetter<Context, Tag> {
    /// Maps the field's value.
    fn map_field<T>(
        context: &Context,
        _tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T;
}

impl<Getter, Context, Tag> FieldMapper<Context, Tag> for Getter
where
    Getter: FieldGetter<Context, Tag> + 'static,
    Tag: 'static,
{
    fn map_field<T>(
        context: &Context,
        tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T {
        mapper(Getter::get_field(context, tag))
    }
}
