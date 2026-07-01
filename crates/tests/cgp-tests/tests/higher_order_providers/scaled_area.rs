//! The scaling pattern wired onto several contexts: a base calculator, and the
//! higher-order `ScaledAreaCalculator<Inner>` wrapping it.
//!
//! Each context picks its base calculator and, for the scaled variants, wraps it
//! with `ScaledAreaCalculator`. The wiring is a plain
//! `delegate_and_check_components!` compile+wiring check (the macro expansions are
//! owned by `basic_delegation` and `checking`); the runtime test confirms the
//! composed results.
//!
//! See docs/concepts/higher-order-providers.md.

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

#[cgp_impl(new ScaledAreaCalculator<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        let base_area = InnerCalculator::area(self);

        base_area * scale_factor * scale_factor
    }
}

#[derive(HasField)]
pub struct PlainRectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    PlainRectangle {
        AreaCalculatorComponent:
            RectangleAreaCalculator,
    }
}

#[derive(HasField)]
pub struct ScaledRectangle {
    pub scale_factor: f64,
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}

#[derive(HasField)]
pub struct PlainCircle {
    pub radius: f64,
}

delegate_and_check_components! {
    PlainCircle {
        AreaCalculatorComponent:
            CircleAreaCalculator,
    }
}

#[derive(HasField)]
pub struct ScaledCircle {
    pub scale_factor: f64,
    pub radius: f64,
}

delegate_and_check_components! {
    ScaledCircle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<CircleAreaCalculator>,
    }
}

#[test]
fn test_scaled_area() {
    let rectangle = PlainRectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);

    let scaled_rectangle = ScaledRectangle {
        scale_factor: 2.0,
        width: 3.0,
        height: 4.0,
    };

    let circle = PlainCircle { radius: 3.0 };

    assert_eq!(circle.area(), 9.0 * PI);

    assert_eq!(scaled_rectangle.area(), 48.0);

    let scaled_circle = ScaledCircle {
        scale_factor: 2.0,
        radius: 3.0,
    };

    assert_eq!(scaled_circle.area(), 36.0 * PI);
}
