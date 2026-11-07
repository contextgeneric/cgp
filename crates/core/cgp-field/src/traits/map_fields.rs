use crate::traits::MapType;
use crate::types::{Cons, Either, Nil, Void};

/// Maps every field in a list through a type mapper.
///
/// See [`MapFields`](https://contextgeneric.dev/docs/reference/traits/type-level/map_fields).
pub trait MapFields<Mapper> {
    /// The mapped field list.
    type Mapped;
}

impl<Mapper, Current, Rest> MapFields<Mapper> for Cons<Current, Rest>
where
    Mapper: MapType,
    Rest: MapFields<Mapper>,
{
    type Mapped = Cons<Mapper::Map<Current>, Rest::Mapped>;
}

impl<Mapper> MapFields<Mapper> for Nil {
    type Mapped = Nil;
}

impl<Mapper, Current, Rest> MapFields<Mapper> for Either<Current, Rest>
where
    Mapper: MapType,
    Rest: MapFields<Mapper>,
{
    type Mapped = Either<Mapper::Map<Current>, Rest::Mapped>;
}

impl<Mapper> MapFields<Mapper> for Void {
    type Mapped = Void;
}
