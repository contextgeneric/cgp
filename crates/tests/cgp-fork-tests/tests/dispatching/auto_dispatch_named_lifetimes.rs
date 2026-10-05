//! Dispatch methods whose signatures name lifetimes.
//!
//! The generated matcher bound is quantified over the method's own lifetime
//! parameters together with any lifetime the macro names for an elided one, in a
//! single `for<..>`, and never over the trait's lifetime parameters, which the
//! blanket impl already declares. Each trait here needs one of those properties:
//! a named receiver lifetime the return type does not mention, a named receiver
//! beside an elided argument (two lifetimes in one bound), a trait lifetime
//! parameter, and a named receiver whose lifetime an elided return type takes.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

// `HasName::name` names its receiver's lifetime and elides the return's on
// purpose, which is the shape under test.
#![allow(mismatched_lifetime_syntaxes)]

use cgp_fork::prelude::*;

pub struct Circle {
    pub name: String,
}

pub struct Square {
    pub name: String,
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[cgp_auto_dispatch]
pub trait HasSides {
    fn sides<'a>(&'a self) -> u8;
}

#[cgp_auto_dispatch]
pub trait CanLookup {
    fn lookup<'a>(&'a self, key: &str) -> &'a str;
}

#[cgp_auto_dispatch]
pub trait CanPrefix<'a> {
    fn prefix(&self, prefix: &'a str) -> &'a str;
}

#[cgp_auto_dispatch]
pub trait HasName {
    fn name<'a>(&'a self) -> &str;
}

impl HasSides for Circle {
    fn sides<'a>(&'a self) -> u8 {
        0
    }
}

impl HasSides for Square {
    fn sides<'a>(&'a self) -> u8 {
        4
    }
}

impl CanLookup for Circle {
    fn lookup<'a>(&'a self, _key: &str) -> &'a str {
        &self.name
    }
}

impl CanLookup for Square {
    fn lookup<'a>(&'a self, _key: &str) -> &'a str {
        &self.name
    }
}

impl<'a> CanPrefix<'a> for Circle {
    fn prefix(&self, prefix: &'a str) -> &'a str {
        prefix
    }
}

impl<'a> CanPrefix<'a> for Square {
    fn prefix(&self, _prefix: &'a str) -> &'a str {
        "square"
    }
}

impl HasName for Circle {
    fn name<'a>(&'a self) -> &str {
        &self.name
    }
}

impl HasName for Square {
    fn name<'a>(&'a self) -> &str {
        &self.name
    }
}

#[test]
fn test_named_lifetimes() {
    let shape = Shape::Square(Square {
        name: "square".to_owned(),
    });

    assert_eq!(shape.sides(), 4);
    assert_eq!(shape.lookup("key"), "square");
    assert_eq!(shape.prefix("a"), "square");
    assert_eq!(shape.name(), "square");
}
