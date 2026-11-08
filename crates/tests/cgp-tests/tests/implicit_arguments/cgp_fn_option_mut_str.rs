//! `#[cgp_fn]` with a mutable `Option<&mut str>` `#[implicit]` argument.
//!
//! A shared `Option<&str>` implicit reads any `Option<U>` whose `U` implements
//! `AsRef<str>`, but that conversion borrows the field shared and has no mutable
//! counterpart. A mutable `Option<&mut str>` therefore keeps the `Option<String>`
//! field and reads it with `.as_deref_mut()`, the same read a getter with a
//! `&mut self` receiver uses, so the body receives an `Option<&mut str>`.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_fn.md and
//! cgp-knowledge-base/cgp/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn shout_nickname(&mut self, #[implicit] nickname: Option<&mut str>) {
        if let Some(nickname) = nickname {
            nickname.make_ascii_uppercase();
        }
    }

    expand_shout_nickname(output) {
        insta::assert_snapshot!(output, @r#"
        ///The trait for the `shout_nickname` operation.
        pub trait ShoutNickname {
            ///The `shout_nickname` operation.
            fn shout_nickname(&mut self);
        }
        impl<__Context__> ShoutNickname for __Context__
        where
            Self: HasFieldMut<
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
            fn shout_nickname(&mut self) {
                let nickname: Option<&mut str> = ::core::option::Option::as_deref_mut(
                    HasFieldMut::get_field_mut(
                        self,
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
                    ),
                );
                if let Some(nickname) = nickname {
                    nickname.make_ascii_uppercase();
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
fn test_shout_nickname() {
    let mut named = Profile {
        nickname: Some("ada".to_owned()),
    };
    named.shout_nickname();
    assert_eq!(named.nickname.as_deref(), Some("ADA"));

    let mut anon = Profile { nickname: None };
    anon.shout_nickname();
    assert_eq!(anon.nickname, None);
}
