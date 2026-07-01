//! `#[cgp_auto_dispatch]` on a `&self` method that also takes extra by-value
//! arguments, which are forwarded unchanged to the dispatched variant impl.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call(&self, _a: u64, _b: bool) -> &'static str;
}

impl CanCall for Foo {
    fn call(&self, _a: u64, _b: bool) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&self, _a: u64, _b: bool) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_multi_args() {
    assert_eq!(FooBar::Foo(Foo).call(42, true), "foo");
}
