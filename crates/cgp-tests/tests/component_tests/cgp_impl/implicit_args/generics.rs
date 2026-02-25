use core::ops::Mul;

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl<Scalar> AreaCalculator
where
    Scalar: Mul<Output = Scalar> + Clone + Into<f64>,
{
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> f64 {
        (width * height).into()
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    CanUseRectangle for Rectangle;
    Rectangle {
        AreaCalculatorComponent:
            RectangleArea,
    }
}
