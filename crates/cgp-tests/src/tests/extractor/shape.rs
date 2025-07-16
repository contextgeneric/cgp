use std::f64::consts::PI;

use cgp::core::field::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::extra::dispatch::{ExtractFieldAndHandle, MatchWithHandlers};
use cgp::extra::handler::HandleFieldValue;
use cgp::prelude::*;

#[derive(Debug, PartialEq, HasFields, FromVariant, ExtractField)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Debug, PartialEq, HasFields, FromVariant, ExtractField)]
pub enum TriangleOnly {
    Triangle(Triangle),
}

#[derive(Debug, PartialEq, HasFields, FromVariant, ExtractField)]
pub enum ShapePlus {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

#[test]
fn test_shape_area() {
    let shape = Shape::Circle(Circle { radius: 5.0 });

    let _area = match shape
        .to_extractor() // PartialShape<IsPresent, IsPresent>
        .extract_field(PhantomData::<symbol!("Circle")>)
    {
        Ok(circle) => PI * circle.radius * circle.radius,
        // PartialShape<IsVoid, IsPresent>
        Err(remainder) => match remainder.extract_field(PhantomData::<symbol!("Rectangle")>) {
            Ok(rectangle) => rectangle.width * rectangle.height,
            // PartialShape<IsVoid, IsVoid>
        },
    };
}

#[test]
fn test_shape_upcast() {
    let shape = Shape::Circle(Circle { radius: 5.0 });
    let shape_plus = shape.upcast(PhantomData::<ShapePlus>);
    assert_eq!(shape_plus, ShapePlus::Circle(Circle { radius: 5.0 }));
}

#[test]
fn test_shape_downcast() {
    let shape = ShapePlus::Circle(Circle { radius: 5.0 });
    assert_eq!(
        shape.downcast(PhantomData::<Shape>).ok(),
        Some(Shape::Circle(Circle { radius: 5.0 }))
    );

    let shape_plus = ShapePlus::Triangle(Triangle {
        base: 3.0,
        height: 4.0,
    });

    let _area = match shape_plus.downcast(PhantomData::<Shape>) {
        Ok(shape) => match shape {
            Shape::Circle(circle) => PI * circle.radius * circle.radius,
            Shape::Rectangle(rectangle) => rectangle.width * rectangle.height,
        },
        // PartialShapePlus<IsPresent, IsVoid, IsVoid>
        Err(remainder) => match remainder.downcast_fields(PhantomData::<TriangleOnly>) {
            Ok(TriangleOnly::Triangle(triangle)) => triangle.base * triangle.height / 2.0,
        },
    };
}

pub trait HasArea {
    fn area(self) -> f64;
}

#[cgp_computer]
fn compute_area<T: HasArea>(shape: T) -> f64 {
    shape.area()
}

impl HasArea for Circle {
    fn area(self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(self) -> f64 {
        self.width * self.height
    }
}

impl HasArea for Triangle {
    fn area(self) -> f64 {
        self.base * self.height / 2.0
    }
}

#[test]
fn test_match_with_handlers() {
    let circle = Shape::Circle(Circle { radius: 5.0 });

    let _area = MatchWithHandlers::<
        Product![
            ExtractFieldAndHandle<symbol!("Circle"), HandleFieldValue<ComputeArea>>,
            ExtractFieldAndHandle<symbol!("Rectangle"), HandleFieldValue<ComputeArea>>,
        ],
    >::compute(&(), PhantomData::<()>, circle);
}
