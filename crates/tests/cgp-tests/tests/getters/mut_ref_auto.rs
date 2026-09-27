//! `#[cgp_auto_getter]` with a `&mut self` receiver returning a plain `&mut u32`:
//! the blanket impl reads the field mutably through `get_field_mut` (bounded by
//! `HasFieldMut`) and returns the borrow with no conversion, the mutable mirror of
//! a plain `&T` getter.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasCount {
        fn count(&mut self) -> &mut u32;
    }

    expand_has_count(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasCount {
            fn count(&mut self) -> &mut u32;
        }
        impl<__Context__> HasCount for __Context__
        where
            __Context__: HasFieldMut<
                Symbol<5, Chars<'c', Chars<'o', Chars<'u', Chars<'n', Chars<'t', Nil>>>>>>,
                Value = u32,
            >,
        {
            fn count(&mut self) -> &mut u32 {
                self.get_field_mut(
                    ::core::marker::PhantomData::<
                        Symbol<
                            5,
                            Chars<'c', Chars<'o', Chars<'u', Chars<'n', Chars<'t', Nil>>>>>,
                        >,
                    >,
                )
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub count: u32,
}

#[test]
pub fn test_mut_ref_auto_getter() {
    let mut context = App { count: 1 };

    *context.count() += 1;

    assert_eq!(context.count, 2);
}
