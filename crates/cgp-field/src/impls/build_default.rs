use crate::impls::{CanBuildFrom, IsNothing, IsPresent};
use crate::traits::{FinalizeBuild, HasBuilder, TransformMap, TransformMapFields};

pub trait CanBuildWithDefault<Source> {
    fn build_with_default(source: Source) -> Self;
}

pub trait CanFinalizeWithDefault {
    type Output;

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
    Builder::Output: FinalizeBuild<Output = Output>,
{
    type Output = Output;

    fn finalize_with_default(self) -> Output {
        self.transform_map_fields().finalize_build()
    }
}

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
