use core::marker::PhantomData;

use crate::{Either, ExtractField, Field, FromVariant, HasFields, Void};

pub trait CanExtractFrom<Extractor>: Sized {
    type Remainder;

    fn extract_from(source: Extractor) -> Result<Self, Self::Remainder>;
}

impl<Context, Extractor, Remainder> CanExtractFrom<Extractor> for Context
where
    Context: HasFields,
    Context::Fields: FieldsExtractor<Context, Extractor, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn extract_from(extractor: Extractor) -> Result<Self, Self::Remainder> {
        Context::Fields::extract_from(extractor)
    }
}

trait FieldsExtractor<Context, Extractor> {
    type Remainder;

    fn extract_from(extractor: Extractor) -> Result<Context, Self::Remainder>;
}

impl<Context, Tag, Value, RestFields, Extractor, Remainder> FieldsExtractor<Context, Extractor>
    for Either<Field<Tag, Value>, RestFields>
where
    Extractor: ExtractField<Tag, Value = Value>,
    Context: FromVariant<Tag, Value = Value>,
    RestFields: FieldsExtractor<Context, Extractor::Remainder, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn extract_from(extractor: Extractor) -> Result<Context, Remainder> {
        let res = extractor.extract_field(PhantomData);
        match res {
            Ok(field) => Ok(Context::from_variant(PhantomData, field.into())),
            Err(remainder) => RestFields::extract_from(remainder),
        }
    }
}

impl<Context, Extractor> FieldsExtractor<Context, Extractor> for Void {
    type Remainder = Extractor;

    fn extract_from(extractor: Extractor) -> Result<Context, Extractor> {
        Err(extractor)
    }
}
