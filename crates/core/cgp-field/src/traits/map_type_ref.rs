/// Maps a type to a borrowed type.
///
/// See [`MapTypeRef`](https://contextgeneric.dev/docs/reference/traits/type-level/map_type_ref).
pub trait MapTypeRef {
    /// The mapped borrowed type.
    type Map<'a, T: 'a>: 'a;
}
