//! The `#[extend(...)]` + `Self::Scalar` alternative to `#[use_type]` for a
//! `#[cgp_component]`/`#[cgp_impl]` that names abstract types.
//!
//! Instead of importing and rewriting the aliases, `#[extend(HasScalarType,
//! HasErrorType)]` only adds the abstract-type components as supertraits, and the
//! signatures name the types the long way as `Self::Scalar` / `Self::Error`. The
//! provider mirrors this with `#[uses(HasScalarType, HasErrorType)]`. This is the
//! more verbose counterpart to `use_type_component`; `#[use_type]` is the
//! preferred form. Wiring/check and `#[cgp_type]` are incidental scaffolding here.
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
    Rectangle {
        ErrorTypeProviderComponent:
            UseType<Infallible>,
        ScalarTypeProviderComponent:
            UseType<f64>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}
