use crate::Void;

pub trait MapType {
    type Mapped<T>;
}

pub struct IsPresent;

impl MapType for IsPresent {
    type Mapped<T> = T;
}

pub struct IsNothing;

impl MapType for IsNothing {
    type Mapped<T> = ();
}

pub struct IsVoid;

impl MapType for IsVoid {
    type Mapped<T> = Void;
}
