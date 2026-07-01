//! `#[cgp_auto_dispatch]` on a by-value `self` method that takes a `&mut`
//! argument and returns a borrow tied to that argument.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call(self, _a: &mut u64, _b: bool) -> &str;
}

impl CanCall for Foo {
    fn call(self, _a: &mut u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(self, _a: &mut u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_multi_args_owned_self() {
    assert_eq!(FooBar::Foo(Foo).call(&mut 42, true), "foo");
}
