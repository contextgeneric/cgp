use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Clone;
}

#[cgp_auto_getter]
#[use_type(HasScalarType::Scalar)]
pub trait AutoRectangleFields {
    fn width(&self) -> Scalar;

    fn height(&self) -> Scalar;
}

#[cgp_getter(RectangleFieldsGetter)]
#[use_type(HasScalarType::Scalar)]
pub trait HasRectangleFields {
    fn width(&self) -> Scalar;

    fn height(&self) -> Scalar;
}
