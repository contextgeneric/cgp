//! `#[cgp_auto_getter]` on a single-getter trait that declares a local
//! associated type used as the return type: the type is inferred from the field
//! and the trait bound (`Display`) is carried onto the generated blanket impl.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasName {
        type Name: Display;

        fn name(&self) -> &Self::Name;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName {
            type Name: Display;
            fn name(&self) -> &Self::Name;
        }
        impl<__Context__, Name> HasName for __Context__
        where
            Name: Display,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = Name,
            >,
        {
            type Name = Name;
            fn name(&self) -> &Self::Name {
                self.get_field(
                    ::core::marker::PhantomData::<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                )
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

pub trait CheckHasName: HasName<Name = String> {}
impl CheckHasName for Person {}
