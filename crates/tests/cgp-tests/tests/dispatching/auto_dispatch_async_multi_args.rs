//! `#[cgp_auto_dispatch]` combined with `#[async_trait]`: a `&self` async method
//! with extra by-value arguments, dispatched over `FooBar`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall {
    async fn call(&self, _a: u64, _b: bool) -> &'static str;
}

impl CanCall for Foo {
    async fn call(&self, _a: u64, _b: bool) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    async fn call(&self, _a: u64, _b: bool) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_async_multi_args() {
    assert_eq!(block_on(FooBar::Foo(Foo).call(42, true)), "foo");
}
