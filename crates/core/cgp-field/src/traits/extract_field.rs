use core::convert::Infallible;
use core::marker::PhantomData;

use crate::types::Void;

/// Turn an enum into the extractor the variant-narrowing traits operate on, and back, since
/// ExtractField narrows an extractor rather than an enum.
///
/// See [`HasExtractor`](https://contextgeneric.dev/docs/reference/traits/variant/has_extractor).
pub trait HasExtractor {
    /// The extractor type.
    type Extractor;

    /// Converts the value into an extractor.
    fn to_extractor(self) -> Self::Extractor;

    /// Rebuilds the value from an extractor.
    fn from_extractor(extractor: Self::Extractor) -> Self;
}

/// Obtain an extractor that borrows an enum, so its variants can be matched by name with payloads
/// as shared references and the value left intact.
///
/// See
/// [`HasExtractorRef`](https://contextgeneric.dev/docs/reference/traits/variant/has_extractor_ref)
/// .
pub trait HasExtractorRef {
    /// The borrowed extractor type.
    type ExtractorRef<'a>
    where
        Self: 'a;

    /// Borrows the value as an extractor.
    fn extractor_ref(&self) -> Self::ExtractorRef<'_>;
}

/// Obtain an extractor that borrows an enum mutably, so a named variant's payload can be changed in
/// place without consuming or rebuilding the value.
///
/// See
/// [`HasExtractorMut`](https://contextgeneric.dev/docs/reference/traits/variant/has_extractor_mut)
/// .
pub trait HasExtractorMut {
    /// The mutably borrowed extractor type.
    type ExtractorMut<'a>
    where
        Self: 'a;

    /// Borrows the value mutably as an extractor.
    fn extractor_mut(&mut self) -> Self::ExtractorMut<'_>;
}

/// Try one named variant of an enum's extractor, getting its payload or a remainder with that
/// variant ruled out, so a chain ends in a proven-exhaustive match.
///
/// See [`ExtractField`](https://contextgeneric.dev/docs/reference/traits/variant/extract_field).
pub trait ExtractField<Tag> {
    /// The field's value type.
    type Value;

    /// The value left when the field does not match.
    type Remainder;

    /// Extracts the field.
    fn extract_field(self, _tag: PhantomData<Tag>) -> Result<Self::Value, Self::Remainder>;
}

/// Discharge an extractor with every variant ruled out: its type is uninhabited, so the method
/// returns any type and closes the match without a wildcard.
///
/// See
/// [`FinalizeExtract`](https://contextgeneric.dev/docs/reference/traits/variant/finalize_extract) .
pub trait FinalizeExtract {
    /// Finalizes the extraction.
    fn finalize_extract<T>(self) -> T;
}

impl FinalizeExtract for Void {
    fn finalize_extract<T>(self) -> T {
        match self {}
    }
}

impl FinalizeExtract for Infallible {
    fn finalize_extract<T>(self) -> T {
        match self {}
    }
}

/// Collapse the Result the last step of an extraction chain returns, returning the payload and
/// discharging the uninhabited remainder at the type level.
///
/// See
/// [`FinalizeExtractResult`](https://contextgeneric.dev/docs/reference/traits/variant/finalize_extract_result)
/// .
pub trait FinalizeExtractResult {
    /// The extracted value.
    type Output;

    /// Finalizes the result.
    fn finalize_extract_result(self) -> Self::Output;
}

impl<T, E> FinalizeExtractResult for Result<T, E>
where
    E: FinalizeExtract,
{
    type Output = T;

    fn finalize_extract_result(self) -> T {
        match self {
            Ok(value) => value,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}
