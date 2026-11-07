/// Exposes a type's whole field list.
///
/// See [`HasFields`](https://contextgeneric.dev/docs/reference/traits/shape/has_fields).
pub trait HasFields {
    /// The type's field list.
    type Fields;
}

/// Borrows a type's whole field list.
///
/// See [`HasFieldsRef`](https://contextgeneric.dev/docs/reference/traits/shape/has_fields_ref).
pub trait HasFieldsRef {
    /// The borrowed field list.
    type FieldsRef<'a>
    where
        Self: 'a;
}
