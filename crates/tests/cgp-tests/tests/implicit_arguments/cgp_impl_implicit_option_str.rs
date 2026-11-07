//! `#[cgp_impl]` with an `Option<&str>` `#[implicit]` argument.
//!
//! The provider reads any `Option<U>` whose `U` implements `AsRef<str>` through the
//! `OptionAsRef` helper trait, so an `Option<String>`, an `Option<Cow<str>>`, or an
//! `Option<Box<str>>` field works and none needs an allocator.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_impl.md and
//! cgp-knowledge-base/cgp/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

snapshot_cgp_impl! {
    #[cgp_impl(new GreetNickname)]
    impl Greeter {
        fn greet(&self, #[implicit] nickname: Option<&str>) -> String {
            match nickname {
                Some(name) => format!("Hello, {name}!"),
                None => "Hello!".to_owned(),
            }
        }
    }

    expand_greet_nickname(output) {
        insta::assert_snapshot!(output, @r#"
        impl<__Context__> Greeter<__Context__> for GreetNickname
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
                Value: OptionAsRef<str> + 'static,
            >,
        {
            fn greet(__context__: &__Context__) -> String {
                let nickname: Option<&str> = OptionAsRef::option_as_ref(
                    HasField::get_field(
                        __context__,
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
                match nickname {
                    Some(name) => format!("Hello, {name}!"),
                    None => "Hello!".to_owned(),
                }
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for GreetNickname
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
                Value: OptionAsRef<str> + 'static,
            >,
        {}
        ///Provider `GreetNickname` for [`Greeter`].
        pub struct GreetNickname;
        "#)
    }
}
