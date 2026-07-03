//! `#[cgp_fn]` with an `Option<&str>` `#[implicit]` argument.
//!
//! `Option<&str>` is backed by an `Option<String>` field and read with
//! `.as_deref()` — the `&str`/`String` special case composed with the
//! `Option<&T>` reference case — so the body receives an `Option<&str>` while the
//! context stores an `Option<String>`.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn describe(&self, #[implicit] nickname: Option<&str>) -> String {
        match nickname {
            Some(name) => format!("aka {name}"),
            None => "anonymous".to_owned(),
        }
    }

    expand_describe(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait Describe {
            fn describe(&self) -> String;
        }
        impl<__Context__> Describe for __Context__
        where
            Self: HasField<
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
            fn describe(&self) -> String {
                let nickname: Option<&str> = self
                    .get_field(
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
                    .as_deref();
                match nickname {
                    Some(name) => format!("aka {name}"),
                    None => "anonymous".to_owned(),
                }
            }
        }
        "#)
    }
}

#[derive(HasField)]
pub struct Profile {
    pub nickname: Option<String>,
}

#[test]
fn test_describe() {
    let named = Profile {
        nickname: Some("ada".to_owned()),
    };
    assert_eq!(named.describe(), "aka ada");

    let anon = Profile { nickname: None };
    assert_eq!(anon.describe(), "anonymous");
}
