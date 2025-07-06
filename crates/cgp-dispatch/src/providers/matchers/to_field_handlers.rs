use cgp_core::field::{Cons, Either, Field, Nil, Void};

use crate::ExtractFieldAndHandle;

pub trait ToFieldHandlers<Provider> {
    type Handlers;
}

impl<Tag, Value, RestFields, Provider> ToFieldHandlers<Provider>
    for Either<Field<Tag, Value>, RestFields>
where
    RestFields: ToFieldHandlers<Provider>,
{
    type Handlers = Cons<ExtractFieldAndHandle<Tag, Provider>, RestFields::Handlers>;
}

impl<Provider> ToFieldHandlers<Provider> for Void {
    type Handlers = Nil;
}
