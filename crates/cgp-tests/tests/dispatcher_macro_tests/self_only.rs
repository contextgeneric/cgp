use cgp::prelude::*;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

#[cgp_dispatch]
pub trait CanCall {
    fn call(self) -> &'static str;
}

impl CanCall for Foo {
    fn call(self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(self) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_only() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
