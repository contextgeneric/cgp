use core::convert::Infallible;
use core::marker::PhantomData;

use crate::Void;

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
