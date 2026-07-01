//! `#[implicit]` arguments inside a `#[cgp_impl]` provider.
//!
//! A provider method can take `#[implicit]` args just like `#[cgp_fn]`; the
//! provider then depends on the context's fields. The `#[cgp_impl]` expansion is
//! snapshotted in `basic_delegation`, so here the wiring is a plain
//! `delegate_and_check_components!` compile check.
//!
//! See docs/reference/macros/cgp_impl.md and docs/reference/attributes/implicit.md.

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

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    Rectangle {
        AreaCalculatorComponent:
            RectangleArea,
    }
}
