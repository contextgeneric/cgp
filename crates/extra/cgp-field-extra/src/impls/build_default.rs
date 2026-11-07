use cgp_field::impls::{CanBuildFrom, IsNothing, IsOptional, IsPresent};
use cgp_field::traits::{FinalizeBuild, HasBuilder, TransformMap, TransformMapFields};

/// Build a wider record from a narrower one in one call, copying the source's fields by name and
/// filling the rest from Default.
///
/// See
/// [`CanBuildWithDefault`](https://contextgeneric.dev/docs/reference/traits/optional/can_build_with_default)
/// .
pub trait CanBuildWithDefault<Source> {
    /// Builds the value.
    fn build_with_default(source: Source) -> Self;
}

/// End a build by filling every unset field from Default, for an optional builder or a core one,
/// then finalizing through the strict impl.
///
/// See
/// [`CanFinalizeWithDefault`](https://contextgeneric.dev/docs/reference/traits/optional/can_finalize_with_default)
/// .
pub trait CanFinalizeWithDefault {
    /// The finalized value.
    type Output;

    /// Finalizes the builder.
    fn finalize_with_default(self) -> Self::Output;
}

impl<Source, Target, Builder> CanBuildWithDefault<Source> for Target
where
    Target: HasBuilder<Builder = Builder>,
    Builder: CanBuildFrom<Source>,
    Builder::Output: CanFinalizeWithDefault<Output = Target>,
{
    fn build_with_default(source: Source) -> Target {
        Target::builder().build_from(source).finalize_with_default()
    }
}

impl<Builder, Output> CanFinalizeWithDefault for Builder
where
    Builder: TransformMapFields<TransformMapDefault, IsPresent>,
    Builder::Output: FinalizeBuild<Target = Output>,
{
    type Output = Output;

    fn finalize_with_default(self) -> Output {
        self.transform_map_fields().finalize_build()
    }
}

/// The transform marker behind the defaulting finalize: three per-field conversions that leave
/// every field present, filling gaps from Default.
///
/// See
/// [`TransformMapDefault`](https://contextgeneric.dev/docs/reference/traits/optional/transform_map_default)
/// .
pub struct TransformMapDefault;

impl<T> TransformMap<IsPresent, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(value: T) -> T {
        value
    }
}

impl<T: Default> TransformMap<IsNothing, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(_value: ()) -> T {
        T::default()
    }
}

impl<T: Default> TransformMap<IsOptional, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(value: Option<T>) -> T {
        value.unwrap_or_default()
    }
}
