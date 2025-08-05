use core::convert::Infallible;
use core::marker::PhantomData;

use crate::{IsMut, IsRef, MapTypeRef, Void};

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

pub trait HasExtractorAnyRef<M: MapTypeRef>: Sized {
    type ExtractorAnyRef<'a>
    where
        Self: 'a;

    fn extractor_any_ref<'a>(context: M::Map<'a, Self>) -> Self::ExtractorAnyRef<'a>
    where
        Self: 'a;
}

impl<Context> HasExtractorAnyRef<IsRef> for Context
where
    Context: HasExtractorRef,
{
    type ExtractorAnyRef<'a>
        = Context::ExtractorRef<'a>
    where
        Self: 'a;

    fn extractor_any_ref<'a>(context: &'a Context) -> Self::ExtractorAnyRef<'a>
    where
        Self: 'a,
    {
        context.extractor_ref()
    }
}

impl<Context> HasExtractorAnyRef<IsMut> for Context
where
    Context: HasExtractorMut,
{
    type ExtractorAnyRef<'a>
        = Context::ExtractorMut<'a>
    where
        Self: 'a;

    fn extractor_any_ref<'a>(context: &'a mut Context) -> Self::ExtractorAnyRef<'a>
    where
        Self: 'a,
    {
        context.extractor_mut()
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
