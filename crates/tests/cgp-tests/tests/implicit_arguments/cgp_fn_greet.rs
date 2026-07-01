//! `#[cgp_fn]` with a single `#[implicit]` `&str` argument.
//!
//! The implicit `name` is dropped from the signature and read from the context's
//! `name` field via `HasField`, with `.as_str()` applied automatically. The
//! `CheckPerson` bound proves any context with a `name: String` field implements
//! the generated trait.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn greet(&self, #[implicit] name: &str) {
        println!("Hello, {}!", name);
    }

    expand_greet(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait Greet {
            fn greet(&self);
        }
        impl<__Context__> Greet for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = String,
            >,
        {
            fn greet(&self) {
                let name: &str = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .as_str();
                println!("Hello, {}!", name);
            }
        }
        "#)
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

pub trait CheckPerson: Greet {}
impl CheckPerson for Person {}
