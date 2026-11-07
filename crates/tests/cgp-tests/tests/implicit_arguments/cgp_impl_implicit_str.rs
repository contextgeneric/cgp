//! `#[cgp_impl]` with a `&str` `#[implicit]` argument.
//!
//! The provider reads any `AsRef<str>` field through `.as_ref()`, so a `String`, a
//! `Cow<str>`, or a `Box<str>` field works and none needs an allocator. The bound
//! lands on the provider's `IsProviderFor` impl, like any other implicit argument.
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
    #[cgp_impl(new GreetHello)]
    impl Greeter {
        fn greet(&self, #[implicit] name: &str) -> String {
            format!("Hello, {name}!")
        }
    }

    expand_greet_hello(output) {
        insta::assert_snapshot!(output, @r#"
        impl<__Context__> Greeter<__Context__> for GreetHello
        where
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value: ::core::convert::AsRef<str> + 'static,
            >,
        {
            fn greet(__context__: &__Context__) -> String {
                let name: &str = ::core::convert::AsRef::as_ref(
                    HasField::get_field(
                        __context__,
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    ),
                );
                format!("Hello, {name}!")
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for GreetHello
        where
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value: ::core::convert::AsRef<str> + 'static,
            >,
        {}
        ///Provider `GreetHello` for [`Greeter`].
        pub struct GreetHello;
        "#)
    }
}
