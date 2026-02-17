use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(HasScalarType::Scalar)]
pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Clone,
{
    width * height
}
