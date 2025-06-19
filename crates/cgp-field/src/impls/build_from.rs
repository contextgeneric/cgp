use core::marker::PhantomData;

use crate::{BuildField, Cons, Field, HasFields, IntoBuilder, Nil, TakeField};

pub trait CanBuildFrom<Source>: Sized {
    type Output;

    fn build_from(self, source: Source) -> Self::Output;
}

impl<Builder, Source, Output> CanBuildFrom<Source> for Builder
where
    Source: HasFields + IntoBuilder,
    Source::Fields: FieldsBuilder<Source::Builder, Builder, Output = Output>,
{
    type Output = Output;

    fn build_from(self, source: Source) -> Output {
        Source::Fields::build_field(source.into_builder(), self)
    }
}

trait FieldsBuilder<Source, Target> {
    type Output;

    fn build_field(source: Source, target: Target) -> Self::Output;
}

impl<Source, Target, RestFields, Tag, Value> FieldsBuilder<Source, Target>
    for Cons<Field<Tag, Value>, RestFields>
where
    Source: TakeField<Tag, Value = Value>,
    Target: BuildField<Tag, Value = Value>,
    RestFields: FieldsBuilder<Source::Remainder, Target::Output>,
{
    type Output = RestFields::Output;

    fn build_field(source: Source, target: Target) -> Self::Output {
        let (value, next_source) = source.take_field(PhantomData);
        let next_target = target.build_field(PhantomData, value);

        RestFields::build_field(next_source, next_target)
    }
}

impl<Source, Target> FieldsBuilder<Source, Target> for Nil {
    type Output = Target;

    fn build_field(_source: Source, target: Target) -> Self::Output {
        target
    }
}
