pub trait HasBuilder {
    type Builder;

    fn builder() -> Self::Builder;
}

pub trait IntoBuilder {
    type Builder;

    fn into_builder(self) -> Self::Builder;
}
