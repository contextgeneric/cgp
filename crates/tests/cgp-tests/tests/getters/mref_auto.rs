//! `#[cgp_auto_getter]` returning `MRef<'_, String>` (an owned-or-borrowed
//! value): the blanket impl reads a `String` field named after the method and
//! wraps the borrow in `MRef::Ref` without wiring.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::core::field::types::MRef;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> MRef<'_, String>;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&self) -> MRef<'_, String>;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = String,
            >,
        {
            fn foo(&self) -> MRef<'_, String> {
                MRef::Ref(
                    self
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                            >,
                        ),
                )
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: String,
}

#[test]
pub fn test_mref_auto_getter() {
    let context = App { foo: "foo".into() };

    assert_eq!(context.foo().as_ref(), "foo");
}
