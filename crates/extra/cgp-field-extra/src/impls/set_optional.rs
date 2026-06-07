use core::marker::PhantomData;

use cgp_field::impls::IsOptional;
use cgp_field::traits::UpdateField;

pub trait SetOptional<Tag> {
    type Value;

    fn set(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self;

    fn set_optional(
        self,
        _tag: PhantomData<Tag>,
        value: Self::Value,
    ) -> (Option<Self::Value>, Self);
}

impl<Context, Tag> SetOptional<Tag> for Context
where
    Context: UpdateField<Tag, IsOptional, Mapper = IsOptional, Output = Context>,
{
    type Value = Context::Value;

    fn set(self, tag: PhantomData<Tag>, value: Self::Value) -> Self {
        self.set_optional(tag, value).1
    }

    fn set_optional(
        self,
        tag: PhantomData<Tag>,
        value: Self::Value,
    ) -> (Option<Self::Value>, Self) {
        self.update_field(tag, Some(value))
    }
}
