use core::marker::PhantomData;

use crate::{FieldGetter, FieldMapper};

pub struct ChainGetters<GetterA, GetterB>(pub PhantomData<(GetterA, GetterB)>);

impl<Context, Tag, GetterA, GetterB, ValueA, ValueB> FieldGetter<Context, Tag>
    for ChainGetters<GetterA, GetterB>
where
    GetterA: FieldMapper<Context, Tag, Value = ValueA>,
    GetterB: FieldGetter<ValueA, Tag, Value = ValueB>,
{
    type Value = ValueB;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &ValueB {
        GetterA::map_field(context, tag, |value| GetterB::get_field(value, tag))
    }
}
