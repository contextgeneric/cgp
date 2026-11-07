/// Maps a type to another type.
///
/// See [`MapType`](https://contextgeneric.dev/docs/reference/traits/type-level/map_type).
pub trait MapType {
    /// The mapped type.
    type Map<T>;
}
