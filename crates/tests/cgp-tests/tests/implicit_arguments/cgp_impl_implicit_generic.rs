//! `#[implicit]` arguments in a generic `#[cgp_impl]` provider.
//!
//! The provider is generic over `Scalar`, and the implicit `width`/`height` are
//! read as `Scalar` from the context. `#[check_params(f64)]` picks the concrete
//! `Scalar` for the wiring check.
//!
//! See docs/reference/macros/cgp_impl.md and
//! docs/reference/macros/delegate_and_check_components.md.

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
