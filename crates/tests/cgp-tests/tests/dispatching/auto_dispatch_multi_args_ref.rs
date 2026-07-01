//! `#[cgp_auto_dispatch]` on a `&mut self` method with borrowed arguments and a
//! borrowed return, exercising the lifetime handling in the generated handler.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call<'a>(&'a mut self, _a: &'a u64, _b: bool) -> &'a str;
}

impl CanCall for Foo {
    fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_multi_args_ref() {
    assert_eq!(FooBar::Foo(Foo).call(&42, true), "foo");
}
