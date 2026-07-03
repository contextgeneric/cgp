//! `#[cgp_auto_getter]` returning `Option<&str>`: the blanket impl reads an
//! `Option<String>` field named after the method and calls `.as_deref()`,
//! converting `&Option<String>` into `Option<&str>` — the `&str`/`String` special
//! case composed with the option case.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasNickname {
        fn nickname(&self) -> Option<&str>;
    }

    expand_has_nickname(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasNickname {
            fn nickname(&self) -> Option<&str>;
        }
        impl<__Context__> HasNickname for __Context__
        where
            __Context__: HasField<
                Symbol<
                    8,
                    Chars<
                        'n',
                        Chars<
                            'i',
                            Chars<
                                'c',
                                Chars<'k', Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
                Value = Option<String>,
            >,
        {
            fn nickname(&self) -> Option<&str> {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                8,
                                Chars<
                                    'n',
                                    Chars<
                                        'i',
                                        Chars<
                                            'c',
                                            Chars<
                                                'k',
                                                Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .as_deref()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Profile {
    pub nickname: Option<String>,
}

#[test]
pub fn test_option_str_auto_getter() {
    let named = Profile {
        nickname: Some("ada".to_owned()),
    };
    assert_eq!(named.nickname(), Some("ada"));

    let anon = Profile { nickname: None };
    assert_eq!(anon.nickname(), None);
}
