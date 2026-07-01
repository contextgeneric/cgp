//! `#[cgp_auto_dispatch]` combined with `#[async_trait]`: a `&mut self` async
//! method with borrowed arguments and a borrowed return, dispatched over
//! `FooBar`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall {
    async fn call<'a>(&'a mut self, _a: &'a u64, _b: bool) -> &'a str;
}

impl CanCall for Foo {
    async fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    async fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_async_multi_args_ref() {
    assert_eq!(block_on(FooBar::Foo(Foo).call(&42, true)), "foo");
}
