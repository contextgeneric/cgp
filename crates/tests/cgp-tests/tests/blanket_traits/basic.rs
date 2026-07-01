//! The simplest `#[blanket_trait]`: a trait with only supertrait bounds and no
//! body. `#[blanket_trait]` emits the trait unchanged and adds a blanket impl
//! for every `__Context__` that satisfies the supertraits, so any context that
//! is both `Foo` and `Bar` automatically gets `FooBar`.
//!
//! This is the canonical, minimal expansion snapshot for `#[blanket_trait]`.
//! See docs/reference/macros/blanket_trait.md.

use cgp_macro_test_util::snapshot_blanket_trait;

pub trait Foo {}
pub trait Bar {}

snapshot_blanket_trait! {
    #[blanket_trait]
    pub trait FooBar: Foo + Bar {}

    expand_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait FooBar: Foo + Bar {}
        impl<__Context__> FooBar for __Context__
        where
            __Context__: Foo + Bar,
        {}
        ")
    }
}

pub struct Context;

impl Foo for Context {}
impl Bar for Context {}

pub trait CanUseFooBar: FooBar {}
impl CanUseFooBar for Context {}
