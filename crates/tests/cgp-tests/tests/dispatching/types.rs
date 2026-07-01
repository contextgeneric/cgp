//! Shared fixture for the `#[cgp_auto_dispatch]` shape tests.
//!
//! `FooBar` is an extensible-data enum (`#[derive(CgpVariant)]`) over the two
//! unit structs `Foo` and `Bar`. The sibling shape tests each define a
//! `CanCall` trait with a different method shape, implement it for `Foo` and
//! `Bar`, and rely on `#[cgp_auto_dispatch]` to route a `FooBar` value to the
//! matching variant impl. Referenced by siblings via `super::types`.
//!
//! See docs/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;

pub struct Foo;
pub struct Bar;

#[derive(CgpVariant)]
pub enum FooBar {
    Foo(Foo),
    Bar(Bar),
}
