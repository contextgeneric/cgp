use core::fmt::Display;

use cgp::prelude::*;
use futures::executor::block_on;

use crate::dispatcher_macro_tests::types::{Bar, Foo, FooBar};

#[cgp_dispatch]
#[async_trait]
pub trait CanCall<T: Async> {
    async fn call_a(&self, _a: u64, _b: &T) -> String;

    fn call_b(self, _a: u64, b: &mut T) -> &str;
}

impl<T: Display + Async> CanCall<T> for Foo {
    async fn call_a(&self, _a: u64, b: &T) -> String {
        format!("foo-{}", b)
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "foo"
    }
}

impl<T: Async> CanCall<T> for Bar {
    async fn call_a(&self, _a: u64, _b: &T) -> String {
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
    assert_eq!(block_on(FooBar::Foo(Foo).call_a(42, &"extra")), "foo-extra");
}
