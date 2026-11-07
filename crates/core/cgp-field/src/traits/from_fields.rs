use crate::traits::HasFields;

/// Builds a value from its field list.
///
/// See [`FromFields`](https://contextgeneric.dev/docs/reference/traits/shape/from_fields).
pub trait FromFields: HasFields {
    /// Builds the value from the fields.
    fn from_fields(fields: Self::Fields) -> Self;
}
