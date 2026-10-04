//! A dispatch trait whose method has a raw identifier as its name, `r#type`. The
//! generated computer and helper are named from the unrawed name (`ComputeType` and
//! `__compute_type__`), since `r#` cannot appear inside a longer identifier.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp::prelude::*;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

pub struct Circle;

pub struct Square;

#[cgp_auto_dispatch]
pub trait HasKind {
    fn r#type(&self) -> &'static str;
}

impl HasKind for Circle {
    fn r#type(&self) -> &'static str {
        "circle"
    }
}

impl HasKind for Square {
    fn r#type(&self) -> &'static str {
        "square"
    }
}

#[test]
fn test_raw_method_name_dispatches() {
    assert_eq!(Shape::Circle(Circle).r#type(), "circle");
    assert_eq!(Shape::Square(Square).r#type(), "square");
}
