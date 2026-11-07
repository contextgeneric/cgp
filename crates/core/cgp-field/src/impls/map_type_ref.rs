use crate::traits::MapTypeRef;

/// Maps a type to a shared reference.
pub struct IsRef;

impl MapTypeRef for IsRef {
    type Map<'a, T: 'a> = &'a T;
}

/// Maps a type to a mutable reference.
pub struct IsMut;

impl MapTypeRef for IsMut {
    type Map<'a, T: 'a> = &'a mut T;
}

/// Maps a type to itself.
pub struct IsOwned;

impl MapTypeRef for IsOwned {
    type Map<'a, T: 'a> = T;
}
