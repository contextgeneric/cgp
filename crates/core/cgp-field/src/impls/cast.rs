use core::marker::PhantomData;

use crate::traits::{
    ExtractField, FinalizeExtract, FinalizeExtractResult, FromVariant, HasExtractor, HasFields,
};
use crate::types::{Either, Field, Void};

/// Widens a smaller enum into a larger one.
///
/// See [`CanUpcast`](https://contextgeneric.dev/docs/reference/traits/casting/can_upcast).
pub trait CanUpcast<Target> {
    /// Widens the value into the target.
    fn upcast(self, _tag: PhantomData<Target>) -> Target;
}

/// Narrows an enum into a smaller one.
///
/// See [`CanDowncast`](https://contextgeneric.dev/docs/reference/traits/casting/can_downcast).
pub trait CanDowncast<Target> {
    /// The value left when the target does not match.
    type Remainder;

    /// Narrows the value into the target.
    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}

/// Narrows a field list into a smaller one.
///
/// See [`CanDowncastFields`](https://contextgeneric.dev/docs/reference/traits/casting/can_downcast_fields).
pub trait CanDowncastFields<Target> {
    /// The fields left when the target does not match.
    type Remainder;

    /// Narrows the fields into the target.
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

/// Extracts a target's fields from a source.
pub trait FieldsExtractor<Source, Target> {
    /// The source left when the target does not match.
    type Remainder;

    /// Extracts the target from the source.
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
