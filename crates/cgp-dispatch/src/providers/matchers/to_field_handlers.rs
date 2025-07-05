use cgp_core::prelude::*;

use crate::ExtractFieldAndHandle;

pub trait ToInputFieldHandlers<Provider> {
    type Handlers;
}

impl<Input, Fields, Provider> ToInputFieldHandlers<Provider> for Input
where
    Input: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<Provider>,
{
    type Handlers = Fields::Handlers;
}

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
