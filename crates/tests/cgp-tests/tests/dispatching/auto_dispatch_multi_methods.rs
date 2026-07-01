//! `#[cgp_auto_dispatch]` on a trait with multiple methods of differing
//! receiver shapes (`&self`, `&mut self`, `self`), all dispatched over the same
//! `FooBar` enum.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call_a(&self, _a: u64, _b: bool) -> &str;

    fn call_b(&mut self, _a: u64, _b: bool) -> u64;

    fn call_c(self, _a: &u64, _b: bool) -> &str;
}

impl CanCall for Foo {
    fn call_a(&self, _a: u64, _b: bool) -> &str {
        "foo"
    }

    fn call_b(&mut self, _a: u64, _b: bool) -> u64 {
        123
    }

    fn call_c(self, _a: &u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call_a(&self, _a: u64, _b: bool) -> &str {
        "bar"
    }

    fn call_b(&mut self, _a: u64, _b: bool) -> u64 {
        456
    }

    fn call_c(self, _a: &u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_multi_methods() {
    assert_eq!(FooBar::Foo(Foo).call_a(42, true), "foo");
}
