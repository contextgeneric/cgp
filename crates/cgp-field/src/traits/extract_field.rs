use core::convert::Infallible;
use core::marker::PhantomData;

use crate::{Either, Void};

pub trait HasExtractor {
    type Extractor;

    fn extractor() -> Self::Extractor;
}

pub trait ExtractField<Tag> {
    type Value;

    type Remainder;

    fn extract_field(self, _tag: PhantomData<Tag>) -> Either<Self::Value, Self::Remainder>;
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
