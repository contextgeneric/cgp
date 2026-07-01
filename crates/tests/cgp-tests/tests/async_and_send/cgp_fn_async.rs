//! Canonical expansion of an async `#[cgp_fn]`.
//!
//! `#[async_trait]` combined with `#[cgp_fn]` turns an `async fn` into an async
//! blanket-impl trait: the generated trait method is `async fn`, both the trait
//! and its blanket impl carry `#[async_trait]`, and `#[implicit]` arguments are
//! still pulled from the context's fields. This is the async variant of the
//! `#[cgp_fn]` expansion, which the `async_and_send` concept owns.
//!
//! See docs/reference/macros/async_trait.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    #[async_trait]
    pub async fn greet(&self, #[implicit] name: &str) -> String {
        format!("Hello, {}!", name)
    }

    expand_greet(output) {
        insta::assert_snapshot!(output, @r#"
        #[async_trait]
        pub trait Greet {
            async fn greet(&self) -> String;
        }
        #[async_trait]
        impl<__Context__> Greet for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = String,
            >,
        {
            async fn greet(&self) -> String {
                let name: &str = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .as_str();
                format!("Hello, {}!", name)
            }
        }
        "#)
    }
}
