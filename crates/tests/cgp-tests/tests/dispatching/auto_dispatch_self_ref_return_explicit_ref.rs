//! `#[cgp_auto_dispatch]` on a `&self` method whose return borrow is written
//! with an *explicit* lifetime (`fn call<'a>(&'a self) -> &'a str`).
//!
//! Companion to `auto_dispatch_self_ref_return_implicit_ref`, which writes the
//! same shape with an elided lifetime.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call<'a>(&'a self) -> &'a str;
}

impl CanCall for Foo {
    fn call(&self) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&self) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_ref_return_explicit_ref() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
