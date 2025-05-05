use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

#[diagnostic::on_unimplemented(
    message = "HasField is not implemented for {Self} with the field: {Tag}",
    note = "You need to add #[derive(HasField)] to {Self} with the given field present in the struct"
)]
pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}

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

pub trait FieldGetter<Context, Tag> {
    type Value;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value;
}

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
