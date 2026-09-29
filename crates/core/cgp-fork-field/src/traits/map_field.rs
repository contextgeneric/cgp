use core::marker::PhantomData;

use crate::traits::{FieldGetter, HasField};

/**
    A helper trait to help organize the lifetime inference in Rust.
    Without this, `Self::Value` would need to be `'static`, as Rust couldn't
    infer the correct lifetime when calling chained field getters, such as
    `context.get_field().get_field()`.
*/
pub trait MapField<Tag>: HasField<Tag> {
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

/**
    The provider trait equivalent of [`MapField`].
*/
pub trait FieldMapper<Context, Tag>: FieldGetter<Context, Tag> {
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
