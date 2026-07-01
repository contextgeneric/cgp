//! `#[use_provider]` on a `#[cgp_impl]`: a higher-order provider parameterized by
//! an inner provider.
//!
//! `ScaledArea<Inner>` takes an inner `AreaCalculator` and scales its result. The
//! inner provider is chosen at wiring time (`ScaledArea<RectangleArea>`), so the
//! same outer provider composes with any base calculator.
//!
//! See docs/reference/attributes/use_provider.md and
//! docs/concepts/higher-order-providers.md.

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[cgp_impl(new ScaledArea<Inner>)]
#[use_provider(Inner: AreaCalculator)]
impl<Inner> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        Inner::area(self) * scale_factor * scale_factor
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl CanCalculateArea for Rectangle {
    fn area(&self) -> f64 {
        RectangleArea::area(self)
    }
}

#[derive(HasField)]
pub struct ScaledRectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

delegate_components! {
    ScaledRectangle {
        AreaCalculatorComponent: ScaledArea<RectangleArea>,
    }
}

#[test]
fn test_scaled_area() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(rectangle.area(), 12.0);

    // The inner `RectangleArea` computes 12.0, then `ScaledArea` scales by 2^2.
    let scaled = ScaledRectangle {
        width: 3.0,
        height: 4.0,
        scale_factor: 2.0,
    };
    assert_eq!(scaled.area(), 48.0);
}
