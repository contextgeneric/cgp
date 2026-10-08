use crate::core::field::traits::MapType;
use crate::core::field::types::Void;

pub struct IsPresent;

impl MapType for IsPresent {
    type Map<T> = T;
}

pub struct IsNothing;

impl MapType for IsNothing {
    type Map<T> = ();
}

pub struct IsVoid;

impl MapType for IsVoid {
    type Map<T> = Void;
}

pub struct IsOptional;

impl MapType for IsOptional {
    type Map<T> = Option<T>;
}
