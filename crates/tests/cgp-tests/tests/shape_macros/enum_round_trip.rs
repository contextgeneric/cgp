//! A value of an `Enum!` type moves in and out of the enum it describes through
//! `ToFields` and `FromFields`, and a value built by hand from `Either` and
//! `Field` rebuilds the enum.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/enum.md.

use cgp::prelude::*;

#[derive(Debug, PartialEq, HasFields)]
pub enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Empty,
}

type ShapeFields = Enum! {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Empty,
};

#[test]
fn an_enum_round_trips_through_its_shape() {
    let fields: ShapeFields = Shape::Rectangle {
        width: 3.0,
        height: 4.0,
    }
    .to_fields();

    let Either::Right(Either::Left(rectangle)) = &fields else {
        panic!("expected the `Rectangle` variant");
    };
    let Cons(width, Cons(height, Nil)) = &rectangle.value;
    assert_eq!((width.value, height.value), (3.0, 4.0));

    assert_eq!(
        Shape::from_fields(fields),
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        }
    );
}

#[test]
fn a_value_built_by_hand_rebuilds_the_enum() {
    let circle: ShapeFields = Either::Left(Field::from(2.0));
    assert_eq!(Shape::from_fields(circle), Shape::Circle(2.0));

    let rectangle: ShapeFields = Either::Right(Either::Left(Field::from(product![
        Field::from(5.0),
        Field::from(6.0),
    ])));
    assert_eq!(
        Shape::from_fields(rectangle),
        Shape::Rectangle {
            width: 5.0,
            height: 6.0,
        }
    );

    let empty: ShapeFields = Either::Right(Either::Right(Either::Left(Field::from(Nil))));
    assert_eq!(Shape::from_fields(empty), Shape::Empty);
}
