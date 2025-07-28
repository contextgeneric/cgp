use core::marker::PhantomData;

use crate::Void;

pub trait MapType {
    type Map<T>;
}

pub trait MapTypeRef {
    type Map<'a, T: 'a>: 'a;
}

pub struct IsPresent;

impl MapType for IsPresent {
    type Map<T> = T;
}

impl MapTypeRef for IsPresent {
    type Map<'a, T: 'a> = T;
}

pub struct IsNothing;

impl MapType for IsNothing {
    type Map<T> = ();
}

impl MapTypeRef for IsNothing {
    type Map<'a, T: 'a> = ();
}

pub struct IsVoid;

impl MapType for IsVoid {
    type Map<T> = Void;
}

impl MapTypeRef for IsVoid {
    type Map<'a, T: 'a> = Void;
}

pub struct IsRef<M>(pub PhantomData<M>);

impl<M> MapTypeRef for IsRef<M>
where
    M: MapTypeRef,
{
    type Map<'a, T: 'a> = &'a M::Map<'a, T>;
}
