use std::ops::Mul;

use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area<Scalar>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Clone,
{
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

pub trait CheckRectangle: RectangleArea<f32> {}
impl CheckRectangle for Rectangle {}
