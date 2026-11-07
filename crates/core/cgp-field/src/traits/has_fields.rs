/// Name a type's whole shape as a single type, every field with its name and type, so generic code
/// can walk a struct or enum it cannot name.
///
/// See [`HasFields`](https://contextgeneric.dev/docs/reference/traits/shape/has_fields).
pub trait HasFields {
    /// The type's field list.
    type Fields;
}

/// Name a type's whole shape with every value borrowed, so generic code can read a struct or enum
/// without consuming it. Derived rather than written.
///
/// See [`HasFieldsRef`](https://contextgeneric.dev/docs/reference/traits/shape/has_fields_ref).
pub trait HasFieldsRef {
    /// The borrowed field list.
    type FieldsRef<'a>
    where
        Self: 'a;
}
