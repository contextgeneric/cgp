use cgp::prelude::*;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

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
fn test_call_self_ref_only() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
