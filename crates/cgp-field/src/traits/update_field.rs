use core::marker::PhantomData;

use crate::MapType;

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
