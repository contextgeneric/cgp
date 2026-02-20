use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasTypes {
    type Types: HasScalarType;
}

#[cgp_fn]
#[use_type(HasTypes::Types, @Types::HasScalarType::Scalar)]
pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Clone,
{
    let res: Scalar = width * height;
    res
}

pub struct Types;

impl HasScalarType for Types {
    type Scalar = f64;
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasTypes for Rectangle {
    type Types = Types;
}

pub trait CheckRectangle: RectangleArea {}
impl CheckRectangle for Rectangle {}
