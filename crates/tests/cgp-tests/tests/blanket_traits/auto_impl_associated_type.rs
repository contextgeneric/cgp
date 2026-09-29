//! `#[cgp_auto_impl]` lifts a local associated type to a generic parameter on
//! the blanket impl and rewrites a supertrait equality that named it through
//! `Self::`. The bound on the associated type moves to the impl's where clause.
//!
//! Snapshot variant: blanket provider re-exporting a supertrait associated type.
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_impl.md.

use cgp_macro_test_util::snapshot_cgp_auto_impl;

pub trait HasFooTypeAt<I> {
    type Foo;
}

pub struct Bar;

snapshot_cgp_auto_impl! {
    #[cgp_auto_impl]
    pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
        type FooBar: Clone;
    }

    expand_has_foo_type_at_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
            type FooBar: Clone;
        }
        impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
        where
            FooBar: Clone,
            __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
        {
            type FooBar = FooBar;
        }
        ")
    }
}

pub struct Context;

#[derive(Clone)]
pub struct FooBar;

impl HasFooTypeAt<Bar> for Context {
    type Foo = FooBar;
}

pub trait CanUseFooTypeAtBar: HasFooTypeAtBar<FooBar = FooBar> {}
impl CanUseFooTypeAtBar for Context {}
