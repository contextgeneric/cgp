//! `#[cgp_auto_dispatch]` on a `&self` method with no extra arguments.
//!
//! The generated handler routes a `&FooBar` reference to the matching variant
//! impl of `CanCall`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call(&self) -> &'static str;
}

impl CanCall for Foo {
    fn call(&self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&self) -> &'static str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_ref_only() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
