//! `#[cgp_auto_getter]` with a `&mut self` receiver returning `Option<&mut u32>`:
//! the blanket impl reads an `Option<u32>` field mutably through `get_field_mut`
//! and calls `.as_mut()`, the mutable mirror of the shared `Option<&T>` getter.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&mut self) -> Option<&mut u32>;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&mut self) -> Option<&mut u32>;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasFieldMut<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = Option<u32>,
            >,
        {
            fn foo(&mut self) -> Option<&mut u32> {
                self.get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_mut()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: Option<u32>,
}

#[test]
pub fn test_mut_option_auto_getter() {
    let mut context = App { foo: Some(1) };

    if let Some(value) = context.foo() {
        *value = 42;
    }

    assert_eq!(context.foo, Some(42));
}
