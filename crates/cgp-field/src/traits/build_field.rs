use core::marker::PhantomData;

pub trait HasBuilder {
    type Builder;

    fn builder() -> Self::Builder;
}

pub trait IntoBuilder {
    type Builder;

    fn into_builder(self) -> Self::Builder;
}

pub trait BuildField<Tag> {
    type Value;

    type Output;

    fn build_field(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self::Output;
}

pub trait FinalizeBuild {
    type Output;

    fn finalize_build(self) -> Self::Output;
}
