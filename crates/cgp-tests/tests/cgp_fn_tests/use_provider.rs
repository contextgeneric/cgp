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

#[cgp_fn]
#[use_provider(RectangleAreaCalculator: AreaCalculator)]
fn rectangle_area(&self) -> f64 {
    RectangleAreaCalculator::area(self)
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[test]
fn test_use_provider() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.rectangle_area(), 12.0);
}
