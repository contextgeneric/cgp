use cgp::prelude::*;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

#[cgp_dispatch]
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
fn test_call_self_only() {
    assert_eq!(FooBar::Foo(Foo).call(&42, true), "foo");
}
