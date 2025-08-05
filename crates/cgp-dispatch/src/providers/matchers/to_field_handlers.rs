use cgp_core::prelude::*;

use crate::{ExtractFieldAndHandle, ExtractFirstFieldAndHandle};

pub trait HasFieldHandlers<M> {
    type Handlers;
}

pub trait MapFieldHandler {
    type FieldHandler<Tag>;
}

impl<Context, Fields, M> HasFieldHandlers<M> for Context
where
    Context: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<M>,
{
    type Handlers = Fields::Handlers;
}

pub trait ToFieldHandlers<M> {
    type Handlers;
}

impl<Tag, Value, RestFields, M> ToFieldHandlers<M> for Either<Field<Tag, Value>, RestFields>
where
    M: MapFieldHandler,
    RestFields: ToFieldHandlers<M>,
{
    type Handlers = Cons<M::FieldHandler<Tag>, RestFields::Handlers>;
}

impl<Provider> ToFieldHandlers<Provider> for Void {
    type Handlers = Nil;
}

pub struct MapExtractFieldAndHandle<Provider>(pub PhantomData<Provider>);

impl<Provider> MapFieldHandler for MapExtractFieldAndHandle<Provider> {
    type FieldHandler<Tag> = ExtractFieldAndHandle<Tag, Provider>;
}

pub struct MapExtractFirstFieldAndHandle<Provider>(pub PhantomData<Provider>);

impl<Provider> MapFieldHandler for MapExtractFirstFieldAndHandle<Provider> {
    type FieldHandler<Tag> = ExtractFirstFieldAndHandle<Tag, Provider>;
}
