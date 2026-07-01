//! `#[cgp_auto_getter]` returning `Option<&String>`: the blanket impl reads an
//! `Option<String>` field named after the method and calls `.as_ref()`,
//! converting `&Option<String>` into `Option<&String>` without wiring.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> Option<&String>;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&self) -> Option<&String>;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = Option<String>,
            >,
        {
            fn foo(&self) -> Option<&String> {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_ref()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: Option<String>,
}

#[test]
pub fn test_option_auto_getter() {
    let context = App {
        foo: Some("foo".to_owned()),
    };

    assert_eq!(context.foo(), Some(&"foo".to_owned()));
}
