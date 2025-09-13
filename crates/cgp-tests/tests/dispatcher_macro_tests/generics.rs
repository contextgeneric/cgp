use core::fmt::Display;

use cgp::prelude::*;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall<T> {
    fn call_a(&self, _a: u64, _b: &T) -> String;

    fn call_b(self, _a: u64, b: &mut T) -> &str;
}

impl<T: Display> CanCall<T> for Foo {
    fn call_a(&self, _a: u64, b: &T) -> String {
        format!("foo-{}", b)
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "foo"
    }
}

impl<T> CanCall<T> for Bar {
    fn call_a(&self, _a: u64, _b: &T) -> String {
        "bar".to_owned()
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall<String> {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_only() {
    assert_eq!(FooBar::Foo(Foo).call_a(42, &"extra"), "foo-extra");
}
