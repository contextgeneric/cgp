use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait HasArea {
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

#[cgp_impl(Rectangle)]
#[use_provider(RectangleArea: AreaCalculator)]
impl AreaCalculator for Rectangle {
    fn area(&self) -> f64 {
        #[use_provider(RectangleArea)]
        self.area()
    }
}
