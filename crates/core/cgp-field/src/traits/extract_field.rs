use core::convert::Infallible;
use core::marker::PhantomData;

use crate::types::Void;

pub trait HasExtractor {
    type Extractor;

    fn to_extractor(self) -> Self::Extractor;

    fn from_extractor(extractor: Self::Extractor) -> Self;
}

pub trait HasExtractorRef {
    type ExtractorRef<'a>
    where
        Self: 'a;

    fn extractor_ref(&self) -> Self::ExtractorRef<'_>;
}

pub trait HasExtractorMut {
    type ExtractorMut<'a>
    where
        Self: 'a;

    fn extractor_mut(&mut self) -> Self::ExtractorMut<'_>;
}

pub trait ExtractField<Tag> {
    type Value;

    type Remainder;

    fn extract_field(self, _tag: PhantomData<Tag>) -> Result<Self::Value, Self::Remainder>;
}

pub trait FinalizeExtract {
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

pub trait FinalizeExtractResult {
    type Output;

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
