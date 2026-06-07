use core::f64;
use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Mul<Output = Self::Scalar> + Copy;
}

#[cgp_fn]
#[extend(HasScalarType)]
pub fn rectangle_area(
    &self,
    #[implicit] width: Self::Scalar,
    #[implicit] height: Self::Scalar,
) -> Self::Scalar {
    width * height
}

#[derive(HasField)]
pub struct F32Rectangle {
    pub width: f32,
    pub height: f32,
}

impl HasScalarType for F32Rectangle {
    type Scalar = f32;
}

#[derive(HasField)]
pub struct F64Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for F64Rectangle {
    type Scalar = f64;
}

#[test]
fn test_rectangle_area() {
    let f32_rectangle = F32Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(f32_rectangle.rectangle_area(), 12.0);

    let f64_rectangle = F64Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(f64_rectangle.rectangle_area(), 12.0);
}
