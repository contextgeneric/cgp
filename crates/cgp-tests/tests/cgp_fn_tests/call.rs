use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_fn]
pub fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64
where
    Self: RectangleArea,
{
    self.rectangle_area() * scale_factor * scale_factor
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

pub trait CheckRectangle: ScaledRectangleArea {}
impl CheckRectangle for Rectangle {}
