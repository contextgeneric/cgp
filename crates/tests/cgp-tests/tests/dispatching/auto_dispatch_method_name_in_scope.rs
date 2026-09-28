//! `#[cgp_auto_dispatch]` in a module that already holds a free function with the
//! same name as the trait's method. The per-variant helper function the macro emits
//! takes a reserved name (`__compute_area__`) rather than the method's own, so it
//! does not collide with the module's `area` (`E0428`).
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp::prelude::*;

#[derive(CgpVariant)]
pub enum Shape {
    Square(Square),
}

pub struct Square {
    pub side: f64,
}

// A free function named like the trait's method, in the same module.
pub fn area(side: f64) -> f64 {
    side * side
}

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        area(self.side)
    }
}

#[test]
fn test_method_name_beside_a_free_function() {
    let shape = Shape::Square(Square { side: 3.0 });
    assert_eq!(shape.area(), 9.0);
}
