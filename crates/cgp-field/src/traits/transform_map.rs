use core::marker::PhantomData;

use crate::{Cons, Field, HasFields, IsNothing, MapType, Nil, PartialData, UpdateField};

/// Natural transformation from M1::Map<T> to M2::Map<T>
pub trait TransformMap<M1: MapType, M2: MapType, T> {
    fn transform_mapped(value: M1::Map<T>) -> M2::Map<T>;
}

pub trait TransformMapFields<Transform, TargetMap> {
    type Output;

    fn transform_map_fields(self) -> Self::Output;
}

impl<ContextA, ContextB, Transform, TargetMap, Output> TransformMapFields<Transform, TargetMap>
    for ContextA
where
    ContextA: PartialData<Target = ContextB>,
    ContextB: HasFields,
    ContextB::Fields: TransformMapFieldsImpl<ContextA, Transform, TargetMap, Output = Output>,
{
    type Output = Output;

    fn transform_map_fields(self) -> Self::Output {
        ContextB::Fields::transform_map_fields(self)
    }
}

trait TransformMapFieldsImpl<Context, Transform, TargetMap> {
    type Output;

    fn transform_map_fields(context: Context) -> Self::Output;
}

impl<Context, Transform, TargetMap> TransformMapFieldsImpl<Context, Transform, TargetMap> for Nil {
    type Output = Context;

    fn transform_map_fields(context: Context) -> Self::Output {
        context
    }
}

impl<Tag, Value, Tail, ContextA, ContextB, ContextC, ContextD, Transform, TargetMap, SourceMap>
    TransformMapFieldsImpl<ContextA, Transform, TargetMap> for Cons<Field<Tag, Value>, Tail>
where
    TargetMap: MapType,
    SourceMap: MapType,
    Tail: TransformMapFieldsImpl<ContextA, Transform, TargetMap, Output = ContextB>,
    ContextB: UpdateField<Tag, IsNothing, Value = Value, Mapper = SourceMap, Output = ContextC>,
    ContextC: UpdateField<Tag, TargetMap, Value = Value, Output = ContextD>,
    Transform: TransformMap<SourceMap, TargetMap, Value>,
{
    type Output = ContextD;

    fn transform_map_fields(context_a: ContextA) -> Self::Output {
        let context_b = Tail::transform_map_fields(context_a);

        let (value_a, context_c) = context_b.update_field(PhantomData, ());
        let value_b = Transform::transform_mapped(value_a);
        let (_, context_d) = context_c.update_field(PhantomData, value_b);

        context_d
    }
}
