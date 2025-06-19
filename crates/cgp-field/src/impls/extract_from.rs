use core::marker::PhantomData;

use crate::{Either, ExtractField, Field, FromVariant, HasExtractor, HasFields, Void};

pub trait CanExtractFrom<Source>: Sized {
    type Remainder;

    fn extract_from(source: Source) -> Result<Self, Self::Remainder>;
}

impl<Context, Source, Remainder> CanExtractFrom<Source> for Context
where
    Context: HasFields + DoExtractFrom<Context::Fields, Source::Extractor, Remainder = Remainder>,
    Source: HasExtractor,
{
    type Remainder = Remainder;

    fn extract_from(source: Source) -> Result<Self, Self::Remainder> {
        Context::extract_from(source.extractor())
    }
}

pub trait DoExtractFrom<Fields, Extractor>: Sized {
    type Remainder;

    fn extract_from(extractor: Extractor) -> Result<Self, Self::Remainder>;
}

impl<Context, Tag, Value, RestFields, Extractor>
    DoExtractFrom<Either<Field<Tag, Value>, RestFields>, Extractor> for Context
where
    Extractor: ExtractField<Tag, Value = Value>,
    Context: FromVariant<Tag, Value = Value>,
{
    type Remainder = Extractor::Remainder;

    fn extract_from(extractor: Extractor) -> Result<Self, Self::Remainder> {
        let field = extractor.extract_field(PhantomData)?;
        Ok(Context::from_variant(PhantomData, field.into()))
    }
}

impl<Context, Extractor> DoExtractFrom<Void, Extractor> for Context {
    type Remainder = Extractor;

    fn extract_from(extractor: Extractor) -> Result<Self, Extractor> {
        Err(extractor)
    }
}
