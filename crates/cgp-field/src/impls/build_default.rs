use crate::{
    CanBuildFrom, FinalizeBuild, HasBuilder, IsNothing, IsPresent, TransformMap, TransformMapFields,
};

pub trait CanBuildWithDefault<Source> {
    fn build_with_default(source: Source) -> Self;
}

impl<Source, Target, BuilderA, BuilderB, BuilderC> CanBuildWithDefault<Source> for Target
where
    Target: HasBuilder<Builder = BuilderA>,
    BuilderA: CanBuildFrom<Source, Output = BuilderB>,
    BuilderB: TransformMapFields<TransformMapDefault, IsPresent, Output = BuilderC>,
    BuilderC: FinalizeBuild<Output = Target>,
{
    fn build_with_default(source: Source) -> Target {
        Target::builder()
            .build_from(source)
            .transform_map_fields()
            .finalize_build()
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
