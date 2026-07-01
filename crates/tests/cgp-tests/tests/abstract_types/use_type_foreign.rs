//! Importing an abstract type from a *foreign* type parameter with the
//! `#[use_type(@Types::HasScalarType::Scalar)]` form on a generic component.
//!
//! When the abstract type lives on a generic parameter of the component rather
//! than on `Self`, the `@Types::` prefix tells `#[use_type]` to resolve `Scalar`
//! against that parameter, rewriting the bare alias to
//! `<Types as HasScalarType>::Scalar`. The `Error` type is still resolved against
//! `Self` via `HasErrorType::Error`. `Types` is a standalone type that supplies
//! the concrete scalar, while `Rectangle` supplies the fields and error type.
//! `#[cgp_type]`, wiring, and check are incidental and use the plain macros.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

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
    Scalar: Mul<Output = Scalar> + Copy,
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
    Rectangle {
        AreaCalculatorComponent: Types,
    }
}
