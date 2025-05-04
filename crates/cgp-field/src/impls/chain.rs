use core::marker::PhantomData;

use crate::FieldGetter;

pub struct ChainGetters<GetterA, GetterB>(pub PhantomData<(GetterA, GetterB)>);

impl<Context, Tag, GetterA, GetterB, ValueB> FieldGetter<Context, Tag>
    for ChainGetters<GetterA, GetterB>
where
    Context: for<'a> CanGetChainedFields<'a, Tag, GetterA, GetterB, ValueB = ValueB>,
{
    type Value = ValueB;

    fn get_field<'a>(context: &'a Context, tag: PhantomData<Tag>) -> &'a ValueB {
        let (_, value_b) = context.get_chained_fields(tag);
        value_b
    }
}

trait CanGetChainedFields<'a, Tag, GetterA, GetterB> {
    type ValueA: 'a;
    type ValueB: 'a;

    fn get_chained_fields(&'a self, tag: PhantomData<Tag>) -> (&'a Self::ValueA, &'a Self::ValueB);
}

impl<'a, Context, Tag, GetterA, GetterB> CanGetChainedFields<'a, Tag, GetterA, GetterB> for Context
where
    GetterA: FieldGetter<Context, Tag>,
    GetterB: FieldGetter<GetterA::Value, Tag>,
    GetterA::Value: 'a,
    GetterB::Value: 'a,
{
    type ValueA = GetterA::Value;
    type ValueB = GetterB::Value;

    fn get_chained_fields(&'a self, tag: PhantomData<Tag>) -> (&'a Self::ValueA, &'a Self::ValueB) {
        let value_a = GetterA::get_field(self, tag);
        let value_b = GetterB::get_field(value_a, tag);
        (value_a, value_b)
    }
}
