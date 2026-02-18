use std::convert::Infallible;
use std::ops::Mul;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType::Scalar, HasErrorType::Error)]
pub trait CanCalculateArea {
    fn area(&self) -> Result<Scalar, Error>;
}

#[cgp_impl(new RectangleArea)]
#[use_type(HasScalarType::Scalar, HasErrorType::Error)]
impl AreaCalculator
where
    Scalar: Mul<Output = Scalar> + Clone,
{
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Result<Scalar, Error> {
        Ok(width * height)
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
        ErrorTypeProviderComponent:
            UseType<Infallible>,
        ScalarTypeProviderComponent:
            UseType<f64>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}
