use core::marker::PhantomData;

use crate::{
    Either, ExtractField, Field, FinalizeExtract, FinalizeExtractResult, FromVariant, HasExtractor,
    HasFields, Void,
};

pub trait CanUpcast<Target> {
    fn upcast(self, _tag: PhantomData<Target>) -> Target;
}

pub trait CanDowncast<Target> {
    type Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}

pub trait CanDowncastFields<Target> {
    type Remainder;

    fn downcast_fields(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}

impl<Context, Source, Target, Remainder> CanUpcast<Target> for Context
where
    Context: HasFields + HasExtractor<Extractor = Source>,
    Context::Fields: FieldsExtractor<Source, Target, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    fn upcast(self, _tag: PhantomData<Target>) -> Target {
        Context::Fields::extract_from(self.to_extractor()).finalize_extract_result()
    }
}

impl<Context, Source, Target, Remainder> CanDowncast<Target> for Context
where
    Context: HasExtractor<Extractor = Source>,
    Target: HasFields,
    Target::Fields: FieldsExtractor<Source, Target, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder> {
        Target::Fields::extract_from(self.to_extractor())
    }
}

impl<Source, Target, Remainder> CanDowncastFields<Target> for Source
where
    Target: HasFields,
    Target::Fields: FieldsExtractor<Source, Target, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn downcast_fields(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder> {
        Target::Fields::extract_from(self)
    }
}

pub trait FieldsExtractor<Source, Target> {
    type Remainder;

    fn extract_from(source: Source) -> Result<Target, Self::Remainder>;
}

impl<Source, Target, Tag, Value, RestFields, Remainder> FieldsExtractor<Source, Target>
    for Either<Field<Tag, Value>, RestFields>
where
    Source: ExtractField<Tag, Value = Value>,
    Target: FromVariant<Tag, Value = Value>,
    RestFields: FieldsExtractor<Source::Remainder, Target, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn extract_from(source: Source) -> Result<Target, Remainder> {
        match source.extract_field(PhantomData) {
            Ok(field) => Ok(Target::from_variant(PhantomData, field)),
            Err(remainder) => RestFields::extract_from(remainder),
        }
    }
}

impl<Source, Target> FieldsExtractor<Source, Target> for Void {
    type Remainder = Source;

    fn extract_from(source: Source) -> Result<Target, Source> {
        Err(source)
    }
}
