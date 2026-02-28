use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(@Types::HasScalarType::Scalar)]
pub fn rectangle_area<Types: HasScalarType>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Copy,
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

pub trait CheckRectangle: RectangleArea<Types> {}
impl CheckRectangle for Rectangle {}
