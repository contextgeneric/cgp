//! `#[cgp_auto_getter]` returning `&str`: the blanket impl reads a `String`
//! field named after the method and calls `.as_str()`, exposing a `String` field
//! as `&str` without any wiring.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> &str;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&self) -> &str;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = String,
            >,
        {
            fn foo(&self) -> &str {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_str()
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
pub fn test_string_auto_getter() {
    let context = App {
        foo: "abc".to_owned(),
    };

    assert_eq!(context.foo(), "abc");
}
