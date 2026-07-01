//! Like `associated_type`, but the local associated type carries a bound
//! (`type FooBar: Clone`). The macro moves that bound onto the lifted generic
//! parameter in the blanket impl's `where` clause (`FooBar: Clone`), so the
//! blanket impl only applies when the underlying associated type is `Clone`.
//!
//! Snapshot variant: blanket trait re-exporting a supertrait associated type
//! with a constraint on the local associated type.
//! See docs/reference/macros/blanket_trait.md.

use cgp_macro_test_util::snapshot_blanket_trait;

pub trait HasFooTypeAt<I> {
    type Foo;
}

pub struct Bar;

snapshot_blanket_trait! {
    #[blanket_trait]
    pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
        type FooBar: Clone;
    }

    expand_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
            type FooBar: Clone;
        }
        impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
        where
            __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
            FooBar: Clone,
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
