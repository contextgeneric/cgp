use crate::traits::MapTypeRef;

pub struct IsRef;

impl MapTypeRef for IsRef {
    type Map<'a, T: 'a> = &'a T;
}

pub struct IsMut;

impl MapTypeRef for IsMut {
    type Map<'a, T: 'a> = &'a mut T;
}

pub struct IsOwned;

impl MapTypeRef for IsOwned {
    type Map<'a, T: 'a> = T;
}
