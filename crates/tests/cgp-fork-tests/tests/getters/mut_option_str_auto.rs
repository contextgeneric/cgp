//! `#[cgp_auto_getter]` with a `&mut self` receiver returning `Option<&mut str>`:
//! the blanket impl reads the `Option<String>` field mutably through
//! `get_field_mut` (bounded by `HasFieldMut`) and calls `.as_deref_mut()`, the
//! mutable mirror of `option_str_auto`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_getter.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasName {
        fn name(&mut self) -> Option<&mut str>;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName {
            fn name(&mut self) -> Option<&mut str>;
        }
        impl<__Context__> HasName for __Context__
        where
            __Context__: HasFieldMut<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = Option<String>,
            >,
        {
            fn name(&mut self) -> Option<&mut str> {
                self.get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .as_deref_mut()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub name: Option<String>,
}

#[test]
pub fn test_mut_option_str_auto_getter() {
    let mut context = App {
        name: Some("abc".to_owned()),
    };

    if let Some(name) = context.name() {
        name.make_ascii_uppercase();
    }

    assert_eq!(context.name.as_deref(), Some("ABC"));
}
