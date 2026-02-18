use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Clone;
}

#[cgp_auto_getter]
#[extend(HasScalarType)]
pub trait AutoRectangleFields {
    fn width(&self) -> Self::Scalar;

    fn height(&self) -> Self::Scalar;
}

#[cgp_getter(RectangleFieldsGetter)]
#[extend(HasScalarType)]
pub trait HasRectangleFields {
    fn width(&self) -> Self::Scalar;

    fn height(&self) -> Self::Scalar;
}
