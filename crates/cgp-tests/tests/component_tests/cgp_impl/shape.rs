use core::f64::consts::PI;

use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_fn]
pub fn circle_area(&self, #[implicit] radius: f64) -> f64 {
    PI * radius * radius
}

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

#[cgp_impl(new ScaledRectangleAreaCalculator)]
#[use_provider(RectangleAreaCalculator: AreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        RectangleAreaCalculator::area(self) * scale_factor * scale_factor
    }
}

#[cgp_impl(new ScaledCircleAreaCalculator)]
#[use_provider(CircleAreaCalculator: AreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        CircleAreaCalculator::area(self) * scale_factor * scale_factor
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

#[derive(HasField)]
pub struct PlainRectangle {
    pub width: f64,
    pub height: f64,
}

delegate_components! {
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

delegate_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}

#[derive(HasField)]
pub struct PlainCircle {
    pub radius: f64,
}

delegate_components! {
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

delegate_components! {
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
