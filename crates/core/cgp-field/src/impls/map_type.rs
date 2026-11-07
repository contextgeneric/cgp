use crate::traits::MapType;
use crate::types::Void;

/// Maps a type to itself.
pub struct IsPresent;

impl MapType for IsPresent {
    type Map<T> = T;
}

/// Maps a type to `()`.
pub struct IsNothing;

impl MapType for IsNothing {
    type Map<T> = ();
}

/// Maps a type to [`Void`].
pub struct IsVoid;

impl MapType for IsVoid {
    type Map<T> = Void;
}

/// Maps a type to `Option<T>`.
pub struct IsOptional;

impl MapType for IsOptional {
    type Map<T> = Option<T>;
}
