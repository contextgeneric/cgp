//! A `#[blanket_trait]` that declares its own associated type and ties it to a
//! supertrait's associated type via an equality bound (`Foo = Self::FooBar`).
//! The macro lifts the local associated type into an extra generic parameter on
//! the blanket impl and rewrites the supertrait bound to name it, so the context
//! only needs to implement the underlying `HasFooTypeAt<Bar>`.
//!
//! Snapshot variant: blanket trait re-exporting a supertrait associated type,
//! with no bound on the local associated type.
//! See docs/reference/macros/blanket_trait.md.

use cgp_macro_test_util::snapshot_blanket_trait;

pub trait HasFooTypeAt<I> {
    type Foo;
}

pub struct Bar;

snapshot_blanket_trait! {
    #[blanket_trait]
    pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
        type FooBar;
    }

    expand_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
            type FooBar;
        }
        impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
        where
            __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
        {
            type FooBar = FooBar;
        }
        ")
    }
}

pub struct Context;
pub struct FooBar;

impl HasFooTypeAt<Bar> for Context {
    type Foo = FooBar;
}

pub trait CanUseFooTypeAtBar: HasFooTypeAtBar<FooBar = FooBar> {}
impl CanUseFooTypeAtBar for Context {}
