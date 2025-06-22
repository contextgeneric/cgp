use core::marker::PhantomData;

use crate::{
    Either, ExtractField, Field, FinalizeExtract, FromVariant, HasExtractor, HasFields, Void,
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

impl<Target, Context, Remainder> CanUpcast<Target> for Context
where
    Context: HasFields + HasExtractor,
    Context::Fields: FieldsExtractor<Target, Context::Extractor, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    fn upcast(self, _tag: PhantomData<Target>) -> Target {
        match Context::Fields::extract_from(self.to_extractor()) {
            Ok(target) => target,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

impl<Target, Context, Extractor, Remainder> CanDowncast<Target> for Context
where
    Context: HasExtractor<Extractor = Extractor>,
    Target: HasFields,
    Target::Fields: FieldsExtractor<Target, Extractor, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder> {
        Target::Fields::extract_from(self.to_extractor())
    }
}

impl<Target, Extractor, Remainder> CanDowncastFields<Target> for Extractor
where
    Target: HasFields,
    Target::Fields: FieldsExtractor<Target, Extractor, Remainder = Remainder>,
{
    type Remainder = Remainder;

    fn downcast_fields(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder> {
        Target::Fields::extract_from(self)
    }
}

pub trait FieldsExtractor<Context, Extractor> {
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
