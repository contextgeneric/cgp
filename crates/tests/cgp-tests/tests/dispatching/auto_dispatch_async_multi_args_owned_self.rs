//! `#[cgp_auto_dispatch]` combined with `#[async_trait]`: a by-value `self` async
//! method taking a `&mut` argument and returning a borrow tied to it, dispatched
//! over `FooBar`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall {
    async fn call<'a>(self, _a: &'a mut u64, _b: bool) -> &'a str;
}

impl CanCall for Foo {
    async fn call(self, _a: &mut u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    async fn call(self, _a: &mut u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_async_multi_args_owned_self() {
    assert_eq!(block_on(FooBar::Foo(Foo).call(&mut 42, true)), "foo");
}
