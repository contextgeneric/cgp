use crate::Void;

pub trait MapType {
    type Map<T>;
}

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
