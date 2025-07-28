use cgp_core::prelude::*;

use crate::ExtractFieldAndHandle;

pub trait HasFieldHandlersRef<'a, Provider> {
    type Handlers;
}

impl<'a, Context, Provider, Handlers> HasFieldHandlersRef<'a, Provider> for Context
where
    Context: 'a + HasFieldsRef,
    Context::FieldsRef<'a>: ToFieldHandlers<Provider, Handlers = Handlers>,
{
    type Handlers = Handlers;
}

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
