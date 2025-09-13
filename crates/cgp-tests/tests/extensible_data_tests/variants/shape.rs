use std::f64::consts::PI;

use cgp::core::field::impls::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::core::field::traits::FinalizeExtractResult;
use cgp::extra::dispatch::{
    ExtractFieldAndHandle, ExtractFirstFieldAndHandle, HandleFieldValue, HandleFirstFieldValue,
    MatchFirstWithHandlers, MatchFirstWithValueHandlers, MatchWithHandlers, MatchWithValueHandlers,
};
use cgp::extra::handler::{NoCode, UseInputDelegate};
use cgp::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Debug, PartialEq, CgpData)]
pub enum TriangleOnly {
    Triangle(Triangle),
}

#[derive(Debug, PartialEq, CgpData)]
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
        .extract_field(PhantomData::<Symbol!("Circle")>)
    {
        Ok(circle) => PI * circle.radius * circle.radius,
        // PartialShape<IsVoid, IsPresent>
        Err(remainder) => {
            let rectangle = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();

            rectangle.width * rectangle.height
        }
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
        Err(remainder) => {
            let TriangleOnly::Triangle(triangle) = remainder
                .downcast_fields(PhantomData::<TriangleOnly>)
                .finalize_extract_result();
            triangle.base * triangle.height / 2.0
        }
    };
}

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(self) -> f64;
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
            ExtractFieldAndHandle<Symbol!("Circle"), HandleFieldValue<ComputeArea>>,
            ExtractFieldAndHandle<Symbol!("Rectangle"), HandleFieldValue<ComputeArea>>,
        ],
    >::compute(&(), PhantomData::<()>, circle);
}

pub trait Container {
    fn contains(self, x: f64, y: f64) -> bool;
}

impl Container for Circle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Rectangle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Triangle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Shape {
    fn contains(self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlers::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

impl Container for ShapePlus {
    fn contains(self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlers::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

#[cgp_computer]
fn contains<T: Container>(shape: T, (x, y): (f64, f64)) -> bool {
    shape.contains(x, y)
}

#[test]
fn test_dispatch_contains() {
    let circle = Shape::Circle(Circle { radius: 5.0 });

    let _is_contained = MatchFirstWithHandlers::<
        Product![
            ExtractFirstFieldAndHandle<Symbol!("Circle"), HandleFirstFieldValue<Contains>>,
            ExtractFirstFieldAndHandle<Symbol!("Rectangle"), HandleFirstFieldValue<Contains>>,
        ],
    >::compute(&(), PhantomData::<()>, (circle, (1.0, 2.0)));
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ComputerComponent: UseInputDelegate<new AreaComputers {
            [
                Circle,
                Rectangle,
                Triangle,
            ]:
                ComputeArea,
            [
                Shape,
                ShapePlus,
            ]: MatchWithValueHandlers,
        }>
    }
}

check_components! {
    CanUseApp for App {
        ComputerComponent: [
            ((), Shape),
            ((), ShapePlus),
        ],
    }
}
