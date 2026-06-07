use cgp_field::impls::{IsNothing, IsOptional, IsPresent};
use cgp_field::traits::{HasBuilder, TransformMap, TransformMapFields};

pub trait HasOptionalBuilder {
    type Builder;

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

pub trait ToOptional {
    type Output;

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
