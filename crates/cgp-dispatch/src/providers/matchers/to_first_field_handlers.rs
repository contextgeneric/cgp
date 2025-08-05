use cgp_core::prelude::*;

use crate::ExtractFirstFieldAndHandle;

pub trait HasFirstFieldHandlers<Provider> {
    type Handlers;
}

impl<Context, Fields, Provider> HasFirstFieldHandlers<Provider> for Context
where
    Context: HasFields<Fields = Fields>,
    Fields: ToFirstFieldHandlers<Provider>,
{
    type Handlers = Fields::Handlers;
}

pub trait ToFirstFieldHandlers<Provider> {
    type Handlers;
}

impl<Tag, Value, RestFields, Provider> ToFirstFieldHandlers<Provider>
    for Either<Field<Tag, Value>, RestFields>
where
    RestFields: ToFirstFieldHandlers<Provider>,
{
    type Handlers = Cons<ExtractFirstFieldAndHandle<Tag, Provider>, RestFields::Handlers>;
}

impl<Provider> ToFirstFieldHandlers<Provider> for Void {
    type Handlers = Nil;
}
