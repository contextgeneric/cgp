/// A type that can build itself field by field.
///
/// See [`HasBuilder`](https://contextgeneric.dev/docs/reference/traits/builder/has_builder).
pub trait HasBuilder {
    /// The builder type.
    type Builder;

    /// Creates an empty builder.
    fn builder() -> Self::Builder;
}

/// A type that converts into its builder.
///
/// See [`IntoBuilder`](https://contextgeneric.dev/docs/reference/traits/builder/into_builder).
pub trait IntoBuilder {
    /// The builder type.
    type Builder;

    /// Converts the value into a builder.
    fn into_builder(self) -> Self::Builder;
}
