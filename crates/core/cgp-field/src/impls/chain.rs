use core::marker::PhantomData;

use crate::traits::{FieldGetter, FieldMapper};
use crate::types::{Cons, Nil};

/// The foundational getter that chains a list of field getters, so a getter component reads a field
/// several hops inside its context.
///
/// See [`ChainGetters`](https://contextgeneric.dev/docs/reference/providers/chain_getters).
pub struct ChainGetters<Getters>(pub PhantomData<Getters>);

impl<Context, Tag, Getter, RestGetters, ValueA, ValueB> FieldGetter<Context, Tag>
    for ChainGetters<Cons<Getter, RestGetters>>
where
    Getter: FieldMapper<Context, Tag, Value = ValueA>,
    ChainGetters<RestGetters>: FieldGetter<ValueA, Tag, Value = ValueB>,
{
    type Value = ValueB;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &ValueB {
        Getter::map_field(context, tag, |value| {
            <ChainGetters<RestGetters>>::get_field(value, tag)
        })
    }
}

impl<Context, Tag> FieldGetter<Context, Tag> for ChainGetters<Nil> {
    type Value = Context;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Context {
        context
    }
}
