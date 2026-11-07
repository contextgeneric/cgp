use crate::traits::{HasFields, HasFieldsRef};

/// Produce a value in the shape HasFields names, taking a struct or enum apart into an anonymous
/// list of named entries.
///
/// See [`ToFields`](https://contextgeneric.dev/docs/reference/traits/shape/to_fields).
pub trait ToFields: HasFields {
    /// Converts the value into its fields.
    fn to_fields(self) -> Self::Fields;
}

/// Walk a value's shape without consuming it, for code that only reads the fields of a struct or
/// enum it cannot name.
///
/// See [`ToFieldsRef`](https://contextgeneric.dev/docs/reference/traits/shape/to_fields_ref).
pub trait ToFieldsRef: HasFieldsRef {
    /// Borrows the value as its fields.
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
