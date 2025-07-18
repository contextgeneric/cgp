use crate::{IsNothing, IsPresent, MapType};

pub trait TransformMapFields<Transform, TargetMap> {
    type Output;

    fn transform_map_fields(self) -> Self::Output;
}

/// Natural transformation from M1::Map<T> to M2::Map<T>
pub trait TransformMap<M1: MapType, M2: MapType, T> {
    fn transform_mapped(value: M1::Map<T>) -> M2::Map<T>;
}

pub struct TransformMapDefault;

impl<T> TransformMap<IsPresent, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(value: T) -> T {
        value
    }
}

impl<T: Default> TransformMap<IsNothing, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(_value: ()) -> T {
        T::default()
    }
}