//! Acceptable failure: a higher-order provider whose *inner* layer carries the
//! unmet dependency. `ScaledArea<Inner>` delegates to an inner `AreaCalculator`
//! and adds its own `Self: HasScaleFactor` dependency; `BaseArea` is that inner
//! provider and needs `Self: HasBaseArea`. `Rectangle` supplies `scale_factor`
//! but not `base_area`, so the *outer* layer's dependency holds and the *inner*
//! layer's fails.
//!
//! This fixture pins where the diagnostic locates the failing layer: the
//! `unsatisfied trait bound introduced here` caret lands on `BaseArea`'s
//! `Self: HasBaseArea` clause (the inner provider), and the `required for …` chain
//! runs *through* `ScaledArea<BaseArea>`'s `IsProviderFor` before reaching
//! `CanUseComponent`, so the outer wrapper appears in the chain even though its own
//! bound is satisfied. Contrast higher_order_outer_dependency.rs, whose caret lands
//! on `ScaledArea`'s own clause and whose chain never reaches the inner provider —
//! the two failures are structurally similar but point at different layers, which
//! is why `#[check_providers(...)]` exists to assert `IsProviderFor` per layer.
//! This is the check doing its job, not a macro defect.
//!
//! See docs/errors/checks/higher-order-provider-layer.md.

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_auto_getter]
pub trait HasBaseArea {
    fn base_area(&self) -> f64;
}

#[cgp_auto_getter]
pub trait HasScaleFactor {
    fn scale_factor(&self) -> f64;
}

#[cgp_impl(new BaseArea)]
impl AreaCalculator
where
    Self: HasBaseArea,
{
    fn area(&self) -> f64 {
        self.base_area()
    }
}

#[cgp_impl(new ScaledArea<Inner>)]
#[use_provider(Inner: AreaCalculator)]
impl<Inner> AreaCalculator
where
    Self: HasScaleFactor,
{
    fn area(&self) -> f64 {
        self.scale_factor() * Inner::area(self)
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub scale_factor: f64,
    // missing `base_area`, so the inner `BaseArea` layer fails
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: ScaledArea<BaseArea>,
    }
}

// Fails in the inner `BaseArea` layer: `Rectangle` has `scale_factor` but not
// `base_area`, so `ScaledArea`'s own dependency holds and `BaseArea`'s does not.
check_components! {
    Rectangle {
        AreaCalculatorComponent,
    }
}

fn main() {}
