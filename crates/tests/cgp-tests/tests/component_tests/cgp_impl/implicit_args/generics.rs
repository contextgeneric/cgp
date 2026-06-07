use core::ops::Mul;

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea<Scalar> {
    fn area(&self) -> Scalar;
}

#[cgp_impl(new RectangleArea)]
impl<Scalar> AreaCalculator<Scalar>
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        width * height
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    Rectangle {
        #[check_params(f64)]
        AreaCalculatorComponent:
            RectangleArea,
    }
}
