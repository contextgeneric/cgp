/// The marker trait deciding how a borrowed extractor holds each payload, by shared reference, by
/// mutable reference, or by value, at a lifetime.
///
/// See [`MapTypeRef`](https://contextgeneric.dev/docs/reference/traits/type-level/map_type_ref).
pub trait MapTypeRef {
    /// The mapped borrowed type.
    type Map<'a, T: 'a>: 'a;
}
