use core::marker::PhantomData;

use crate::traits::MapType;

/// The per-field primitive of the builder family: change one field's presence marker and hand back
/// what was stored there before.
///
/// See [`UpdateField`](https://contextgeneric.dev/docs/reference/traits/builder/update_field).
pub trait UpdateField<Tag, M: MapType> {
    /// The field's value type.
    type Value;

    /// The mapper the update returns.
    type Mapper: MapType;

    /// The value with the field updated.
    type Output;

    /// Updates the field.
    fn update_field(
        self,
        _tag: PhantomData<Tag>,
        value: M::Map<Self::Value>,
    ) -> (<Self::Mapper as MapType>::Map<Self::Value>, Self::Output);
}
