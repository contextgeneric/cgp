use crate::traits::{HasFields, HasFieldsRef};

/// Converts a value into its field list.
///
/// See [`ToFields`](https://contextgeneric.dev/docs/reference/traits/shape/to_fields).
pub trait ToFields: HasFields {
    /// Converts the value into its fields.
    fn to_fields(self) -> Self::Fields;
}

/// Borrows a value as its field list.
///
/// See [`ToFieldsRef`](https://contextgeneric.dev/docs/reference/traits/shape/to_fields_ref).
pub trait ToFieldsRef: HasFieldsRef {
    /// Borrows the value as its fields.
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
