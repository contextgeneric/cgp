//! One context can carry the fields for several calculators at once and call each
//! provider explicitly, in addition to its wired `area()`.
//!
//! `IsThisRectangleOrCircle` has `width`, `height`, and `radius`, so both the
//! `RectangleAreaCalculator` and `CircleAreaCalculator` providers — and the
//! `rectangle_area`/`circle_area` `#[cgp_fn]`s — apply to it. The `#[cgp_fn]`
//! expansions are owned by `implicit_arguments`, so they are written plainly here.
//!
//! See docs/concepts/higher-order-providers.md and docs/reference/macros/cgp_fn.md.

use core::f64::consts::PI;

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[cgp_impl(new CircleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] radius: f64) -> f64 {
        PI * radius * radius
    }
}

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_fn]
pub fn circle_area(&self, #[implicit] radius: f64) -> f64 {
    PI * radius * radius
}

#[derive(HasField)]
pub struct IsThisRectangleOrCircle {
    pub width: f64,
    pub height: f64,
    pub radius: f64,
}

impl CanCalculateArea for IsThisRectangleOrCircle {
    fn area(&self) -> f64 {
        CircleAreaCalculator::area(self)
    }
}

#[test]
fn test_rectangle_or_circle() {
    let rectangle_or_circle = IsThisRectangleOrCircle {
        width: 2.0,
        height: 3.0,
        radius: 4.0,
    };

    let area = rectangle_or_circle.area();
    assert_eq!(area, 16.0 * PI);

    let rectangle_area = RectangleAreaCalculator::area(&rectangle_or_circle);
    assert_eq!(rectangle_area, 6.0);

    let circle_area = CircleAreaCalculator::area(&rectangle_or_circle);
    assert_eq!(circle_area, 16.0 * PI);

    let rectangle_area = rectangle_or_circle.rectangle_area();
    assert_eq!(rectangle_area, 6.0);

    let circle_area = rectangle_or_circle.circle_area();
    assert_eq!(circle_area, 16.0 * PI);
}
