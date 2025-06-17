use core::convert::Infallible;
use core::marker::PhantomData;

use crate::{Either, Void};

pub trait ExtractField<Tag> {
    type Value;

    type Remainder;

    fn build_field(self, _tag: PhantomData<Tag>) -> Either<Self::Value, Self::Remainder>;
}

pub trait Discharge {
    fn discharge<T>(self) -> T;
}

impl Discharge for Void {
    fn discharge<T>(self) -> T {
        match self {}
    }
}

impl Discharge for Infallible {
    fn discharge<T>(self) -> T {
        match self {}
    }
}
