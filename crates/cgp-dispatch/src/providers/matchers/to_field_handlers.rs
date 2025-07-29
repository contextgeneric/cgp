use cgp_core::prelude::*;

use crate::ExtractFieldAndHandle;

pub trait HasFieldHandlers<Provider> {
    type Handlers;
}

impl<Context, Fields, Provider> HasFieldHandlers<Provider> for Context
where
    Context: HasFields<Fields = Fields>,
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
