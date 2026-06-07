use cgp::prelude::*;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

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
fn test_call_self_only() {
    assert_eq!(FooBar::Foo(Foo).call(&mut 42, true), "foo");
}
