/// The marker trait whose Map`<T>` decides how one field of a partial record or enum is stored: its
/// value, nothing, an uninhabited type, or an Option.
///
/// See [`MapType`](https://contextgeneric.dev/docs/reference/traits/type-level/map_type).
pub trait MapType {
    /// The mapped type.
    type Map<T>;
}
