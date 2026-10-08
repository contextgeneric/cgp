//! `#[derive_promote]` wraps a `Computer` as a component provider.
//!
//! The component method takes only `&self`. The computer reads its inputs from
//! context fields, and the generated `PromoteAreaCalculator<RectangleArea>`
//! provider implements `AreaCalculator` by calling `Computer::compute` with `()`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_computer.md.

use cgp_fork::prelude::*;

#[cgp_component(AreaCalculator)]
#[derive_promote(PromoteAreaCalculator)]
pub trait HasArea {
    fn area(&self) -> f64;
}

#[cgp_computer]
fn rectangle_area(#[field] width: f64, #[field] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
struct Rectangle {
    width: f64,
    height: f64,
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: PromoteAreaCalculator<RectangleArea>,
    }
}

#[test]
fn test_promoted_area() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);
}
