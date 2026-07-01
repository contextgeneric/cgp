//! `#[cgp_auto_dispatch]` combined with `#[async_trait]`: a by-value `self`
//! async method, dispatched over `FooBar`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall {
    async fn call(self) -> &'static str;
}

impl CanCall for Foo {
    async fn call(self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    async fn call(self) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_async_self_only() {
    assert_eq!(block_on(FooBar::Foo(Foo).call()), "foo");
}
