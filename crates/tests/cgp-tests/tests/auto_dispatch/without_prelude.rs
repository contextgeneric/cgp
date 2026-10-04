//! `#[cgp_auto_dispatch]` in a module that does not import `cgp::prelude::*`.
//!
//! The expansion names the matchers, the computer traits, and `HasExtractor`
//! through their fully qualified `::cgp::macro_prelude` paths, so the macro works
//! when invoked by path alone, and items in the caller's module named like those
//! traits, here a local `Computer` and `HasExtractor`, do not capture them.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

/// Local items named like the traits the expansion refers to.
pub struct Computer;
pub struct HasExtractor;

pub struct Circle;

pub struct Square;

#[derive(cgp::prelude::CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[cgp::prelude::cgp_auto_dispatch]
pub trait HasSides {
    fn sides(&self) -> u8;

    fn scale(&mut self, factor: u8) -> u8;
}

impl HasSides for Circle {
    fn sides(&self) -> u8 {
        0
    }

    fn scale(&mut self, factor: u8) -> u8 {
        factor
    }
}

impl HasSides for Square {
    fn sides(&self) -> u8 {
        4
    }

    fn scale(&mut self, factor: u8) -> u8 {
        4 * factor
    }
}

#[test]
fn test_dispatch_without_prelude() {
    let mut shape = Shape::Square(Square);

    assert_eq!(shape.sides(), 4);
    assert_eq!(shape.scale(2), 8);
}
