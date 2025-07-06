use std::f64::consts::PI;

use cgp::prelude::*;

#[derive(FromVariant, ExtractField)]
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
