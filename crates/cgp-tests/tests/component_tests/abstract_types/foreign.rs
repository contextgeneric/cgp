use std::convert::Infallible;
use std::ops::Mul;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(AreaCalculator)]
#[use_type(@Types::HasScalarType::Scalar, HasErrorType::Error)]
pub trait CanCalculateArea<Types: HasScalarType> {
    fn area(&self) -> Result<Scalar, Error>;
}

#[cgp_impl(new RectangleArea)]
#[use_type(@Types::HasScalarType::Scalar, HasErrorType::Error)]
impl<Types> AreaCalculator<Types>
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

pub struct Types;

impl HasScalarType for Types {
    type Scalar = f64;
}

delegate_components! {
    Rectangle {
        ErrorTypeProviderComponent:
            UseType<Infallible>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}

check_components! {
    CanUseRectangle for Rectangle {
        AreaCalculatorComponent: Types,
    }
}
