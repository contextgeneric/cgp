//! Importing abstract types into a `#[cgp_component]` and `#[cgp_impl]` with
//! `#[use_type(...)]`, and fixing them on a context with `UseType<Concrete>`.
//!
//! `#[use_type(HasScalarType::Scalar, HasErrorType::Error)]` adds the two
//! abstract-type components as supertraits/dependencies and rewrites the bare
//! aliases `Scalar` and `Error` to `<Self as HasScalarType>::Scalar` /
//! `<Self as HasErrorType>::Error`, so the signatures read without any `Self::`
//! qualification. The `Rectangle` context then fixes both types by wiring the
//! respective components to `UseType<f64>` / `UseType<Infallible>`. The
//! `#[cgp_type]` scaffolding and the delegation/check are incidental here and use
//! the plain macros; the abstract-type expansion and wiring are owned elsewhere.
//!
//! See docs/reference/attributes/use_type.md, docs/reference/providers/use_type.md,
//! and docs/concepts/abstract-types.md.

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

delegate_and_check_components! {
    Rectangle {
        ErrorTypeProviderComponent:
            UseType<Infallible>,
        ScalarTypeProviderComponent:
            UseType<f64>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}
