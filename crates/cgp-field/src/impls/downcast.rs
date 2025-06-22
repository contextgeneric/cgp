use core::marker::PhantomData;

use crate::{Either, ExtractField, Field, FromVariant, HasFields, Void};

pub trait CanDowncast<Target>: Sized {
    type Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}

impl<Target, Extractor, Remainder> CanDowncast<Target> for Extractor
where
    Target: HasFields,
    Target::Fields: FieldsExtractor<Target, Extractor, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder> {
        Target::Fields::extract_from(self)
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
            Ok(field) => Ok(Context::from_variant(PhantomData, field)),
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
