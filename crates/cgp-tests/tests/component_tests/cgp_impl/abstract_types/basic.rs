use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea: HasScalarType {
    fn area(&self) -> Self::Scalar;
}

#[cgp_impl(new RectangleArea)]
#[use_type(HasScalarType::Scalar)]
impl AreaCalculator
where
    Scalar: Mul<Output = Scalar> + Clone,
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
    CanUseRectangle for Rectangle;
    Rectangle {
        ScalarTypeProviderComponent:
            UseType<f64>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}
