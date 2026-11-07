//! A `#[cgp_fn]` function with no doc comment.
//!
//! The generated line fills the gap: the method carries `The `greet_fn` operation.`
//! and the trait carries `The trait for the `greet_fn` operation.`, so a crate
//! that enables `missing_docs` reports neither. Together with
//! `doc_comment_on_cgp_fn`, this pins both halves of the rule: a generated line is
//! never appended after a user's doc, and it is added only where the item would
//! otherwise have none.
//!
//! See cgp-knowledge-base/cgp/implementation/README.md#doc-comments-on-generated-items.

use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn greet_fn(&self, #[implicit] name: &str) -> String {
        name.to_owned()
    }

    expand_greet_fn_without_doc(output) {
        insta::assert_snapshot!(output, @"
        ///The trait for the `greet_fn` operation.
        pub trait GreetFn {
            ///The `greet_fn` operation.
            fn greet_fn(&self) -> String;
        }
        impl<__Context__> GreetFn for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value: ::core::convert::AsRef<str> + 'static,
            >,
        {
            fn greet_fn(&self) -> String {
                let name: &str = ::core::convert::AsRef::as_ref(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    ),
                );
                name.to_owned()
            }
        }
        ")
    }
}
