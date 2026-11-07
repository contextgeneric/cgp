/// Start an empty builder for a record, as the entry point to assembling a struct field by field
/// rather than through a struct literal that names the type.
///
/// See [`HasBuilder`](https://contextgeneric.dev/docs/reference/traits/builder/has_builder).
pub trait HasBuilder {
    /// The builder type.
    type Builder;

    /// Creates an empty builder.
    fn builder() -> Self::Builder;
}

/// Turn a complete record into a partial one with every field present, to take fields out and put
/// them back without naming the concrete type.
///
/// See [`IntoBuilder`](https://contextgeneric.dev/docs/reference/traits/builder/into_builder).
pub trait IntoBuilder {
    /// The builder type.
    type Builder;

    /// Converts the value into a builder.
    fn into_builder(self) -> Self::Builder;
}
