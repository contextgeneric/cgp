use std::convert::Infallible;
use std::ops::Mul;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(AreaCalculator)]
#[extend(HasScalarType, HasErrorType)]
pub trait CanCalculateArea {
    fn area(&self) -> Result<Self::Scalar, Self::Error>;
}

#[cgp_impl(new RectangleArea)]
#[uses(HasScalarType, HasErrorType)]
impl AreaCalculator
where
    Self::Scalar: Mul<Output = Self::Scalar> + Copy,
{
    fn area(
        &self,
        #[implicit] width: Self::Scalar,
        #[implicit] height: Self::Scalar,
    ) -> Result<Self::Scalar, Self::Error> {
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
