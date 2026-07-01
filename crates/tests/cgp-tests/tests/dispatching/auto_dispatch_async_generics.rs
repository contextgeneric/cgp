//! `#[cgp_auto_dispatch]` combined with `#[async_trait]` on a generic trait
//! (`CanCall<T>`) mixing an async and a sync method, with a per-variant
//! `T: Display` bound on `Foo`. Dispatched over `FooBar`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use core::fmt::Display;

use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall<T> {
    async fn call_a(&self, _a: u64, _b: &T) -> String;

    fn call_b(self, _a: u64, b: &mut T) -> &str;
}

impl<T: Display> CanCall<T> for Foo {
    async fn call_a(&self, _a: u64, b: &T) -> String {
        format!("foo-{}", b)
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "foo"
    }
}

impl<T> CanCall<T> for Bar {
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
fn test_call_async_generics() {
    assert_eq!(block_on(FooBar::Foo(Foo).call_a(42, &"extra")), "foo-extra");
}
