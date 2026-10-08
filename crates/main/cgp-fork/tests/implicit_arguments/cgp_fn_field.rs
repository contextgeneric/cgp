//! `#[field]` on `#[cgp_fn]` reads a context field the same way `#[implicit]` does.
//!
//! The two attributes are one mechanism: the argument leaves the signature, the
//! impl gains a `HasField` bound, and the body binds the field. This file checks
//! that a context with the named field can call the generated method.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_fn.md and
//! cgp-knowledge-base-fork/cgp/reference/attributes/implicit.md.

use cgp_fork::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[field] width: f64, #[field] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.rectangle_area(), 12.0);
}
