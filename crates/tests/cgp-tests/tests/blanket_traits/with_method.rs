//! A `#[blanket_trait]` carrying a default method. The default method is copied
//! verbatim into the blanket impl, so every qualifying context inherits the
//! method body, which delegates to the supertrait methods (`foo`/`bar`).
//!
//! Snapshot variant: blanket trait with a default method body.
//! See docs/reference/macros/blanket_trait.md.

use cgp_macro_test_util::snapshot_blanket_trait;

pub trait Foo {
    fn foo(&self);
}
pub trait Bar {
    fn bar(&self);
}

snapshot_blanket_trait! {
    #[blanket_trait]
    pub trait FooBar: Foo + Bar {
        fn foo_bar(&self) {
            self.foo();
            self.bar();
        }
    }

    expand_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait FooBar: Foo + Bar {
            fn foo_bar(&self) {
                self.foo();
                self.bar();
            }
        }
        impl<__Context__> FooBar for __Context__
        where
            __Context__: Foo + Bar,
        {
            fn foo_bar(&self) {
                self.foo();
                self.bar();
            }
        }
        ")
    }
}

pub struct Context;

impl Foo for Context {
    fn foo(&self) {}
}

impl Bar for Context {
    fn bar(&self) {}
}

pub trait CanUseFooBar: FooBar {}
impl CanUseFooBar for Context {}
