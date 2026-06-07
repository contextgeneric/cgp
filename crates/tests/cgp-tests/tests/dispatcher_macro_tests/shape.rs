use core::f64::consts::PI;

use cgp::prelude::*;

#[derive(CgpVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[cgp_auto_dispatch]
pub trait CanScale {
    fn scale(&mut self, factor: f64);
}

impl CanScale for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl CanScale for Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

pub trait CheckShapeImpl: HasArea + CanScale {}
impl CheckShapeImpl for Shape {}

#[test]
fn test_shape() {
    let mut shape = Shape::Rectangle(Rectangle {
        width: 2.0,
        height: 2.0,
    });

    assert_eq!(shape.area(), 4.0);

    shape.scale(2.0);
    assert_eq!(shape.area(), 16.0);
}
