use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasTypes {
    type Types;
}

#[cgp_fn]
#[use_type(HasTypes::Types, @Types::HasScalarType::Scalar)]
#[extend_where(Types: HasScalarType)]
pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Clone,
{
    let res: Scalar = width * height;
    res
}

pub struct MyTypes;

impl HasScalarType for MyTypes {
    type Scalar = f64;
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasTypes for Rectangle {
    type Types = MyTypes;
}

pub trait CheckRectangle: RectangleArea
where
    Self::Types: HasScalarType,
{
}

impl CheckRectangle for Rectangle {}
