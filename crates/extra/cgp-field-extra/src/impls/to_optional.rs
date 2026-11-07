use cgp_field::impls::{IsNothing, IsOptional, IsPresent};
use cgp_field::traits::{HasBuilder, TransformMap, TransformMapFields};

/// A type whose builder has optional fields.
///
/// See [`HasOptionalBuilder`](https://contextgeneric.dev/docs/reference/traits/optional/has_optional_builder).
pub trait HasOptionalBuilder {
    /// The optional builder type.
    type Builder;

    /// Creates an empty optional builder.
    fn optional_builder() -> Self::Builder;
}

impl<Context, Builder> HasOptionalBuilder for Context
where
    Context: HasBuilder,
    Context::Builder: ToOptional<Output = Builder>,
{
    type Builder = Builder;

    fn optional_builder() -> Self::Builder {
        Self::builder().to_optional()
    }
}

/// Converts a builder into one with optional fields.
///
/// See [`ToOptional`](https://contextgeneric.dev/docs/reference/traits/optional/to_optional).
pub trait ToOptional {
    /// The optional builder.
    type Output;

    /// Converts the builder.
    fn to_optional(self) -> Self::Output;
}

impl<Context> ToOptional for Context
where
    Context: TransformMapFields<TransformOptional, IsOptional>,
{
    type Output = Context::Output;

    fn to_optional(self) -> Self::Output {
        self.transform_map_fields()
    }
}

/// Maps a present field to an optional one.
///
/// See [`TransformOptional`](https://contextgeneric.dev/docs/reference/traits/optional/transform_optional).
pub struct TransformOptional;

impl<T> TransformMap<IsPresent, IsOptional, T> for TransformOptional {
    fn transform_mapped(value: T) -> Option<T> {
        Some(value)
    }
}

impl<T> TransformMap<IsNothing, IsOptional, T> for TransformOptional {
    fn transform_mapped(_value: ()) -> Option<T> {
        None
    }
}
