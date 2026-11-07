use crate::traits::HasFields;

/// Rebuild a concrete struct or enum from the anonymous shape ToFields produced, as the return
/// journey of a structural operation.
///
/// See [`FromFields`](https://contextgeneric.dev/docs/reference/traits/shape/from_fields).
pub trait FromFields: HasFields {
    /// Builds the value from the fields.
    fn from_fields(fields: Self::Fields) -> Self;
}
