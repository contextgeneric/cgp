use core::marker::PhantomData;

use crate::MapType;

// use crate::{BuildField, IsNothing, IsPresent, MapType, TakeField};

pub trait UpdateField<Tag, M: MapType> {
    type Value;

    type Mapper: MapType;

    type Output;

    fn update_field(
        self,
        _tag: PhantomData<Tag>,
        value: M::Map<Self::Value>,
    ) -> (<Self::Mapper as MapType>::Map<Self::Value>, Self::Output);
}

// impl<Context, Tag> BuildField<Tag> for Context
// where
//     Context: UpdateField<Tag, IsPresent, Mapper = IsNothing>,
// {
//     type Value = Context::Value;

//     type Output = Context::Output;

//     fn build_field(self, tag: PhantomData<Tag>, value: Self::Value) -> Self::Output {
//         self.update_field(tag, value).1
//     }
// }

// impl<Context, Tag> TakeField<Tag> for Context
// where
//     Context: UpdateField<Tag, IsNothing, Mapper = IsPresent>,
// {
//     type Value = Context::Value;

//     type Remainder = Context::Output;

//     fn take_field(self, tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder) {
//         self.update_field(tag, ())
//     }
// }
