//! `#[uses(...)]` on a `#[cgp_impl]` provider imports a `Self` trait bound, read
//! like a `use` statement, so the provider body can call another capability. Here
//! `RectangleAreaCalculator` declares `#[uses(RectangleArea)]` and calls
//! `self.rectangle_area()`; the import becomes an impl-side dependency
//! (`Self: RectangleArea`) that the `CanCalculateArea` consumer trait does not
//! expose. The context satisfies it by having the `width`/`height` fields that
//! `RectangleArea`'s blanket impl requires.
//!
//! The `#[uses(...)]` on the `#[cgp_impl]` is the feature under test and is
//! written plainly. The `#[cgp_fn]` dependency and the `delegate_and_check_components!`
//! wiring are incidental scaffolding (their expansions are owned by the
//! `implicit_arguments`, `basic_delegation`, and `checking` concepts), so both are
//! written as plain macros.
//!
//! See docs/concepts/impl-side-dependencies.md,
//! docs/reference/attributes/uses.md, and docs/reference/macros/cgp_impl.md.

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleAreaCalculator)]
#[uses(RectangleArea)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.rectangle_area()
    }
}

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    Rectangle {
        AreaCalculatorComponent:
            RectangleAreaCalculator,
    }
}

#[test]
fn test_impl_uses() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);
}
