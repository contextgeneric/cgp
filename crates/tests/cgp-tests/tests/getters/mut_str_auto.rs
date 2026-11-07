//! `#[cgp_auto_getter]` with a `&mut self` receiver returning `&mut str`: the
//! blanket impl reads the `String` field mutably through `get_field_mut` (bounded
//! by `HasFieldMut`) and calls `.as_mut()`, so a `String` field can be exposed
//! as a mutable `&mut str`.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&mut self) -> &mut str;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&mut self) -> &mut str;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasFieldMut<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = String,
            >,
        {
            fn foo(&mut self) -> &mut str {
                ::core::convert::AsMut::as_mut(
                    HasFieldMut::get_field_mut(
                        self,
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
pub fn test_mut_str_auto_getter() {
    let mut context = App {
        foo: "abc".to_owned(),
    };

    context.foo().make_ascii_uppercase();

    assert_eq!(context.foo, "ABC");
}
