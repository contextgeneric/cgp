//! `#[cgp_auto_dispatch]` on a `&mut self` method with no extra arguments.
//!
//! The generated handler routes a `&mut FooBar` reference to the matching
//! variant impl of `CanCall`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call(&mut self) -> &'static str;
}

impl CanCall for Foo {
    fn call(&mut self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&mut self) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_mut_only() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
