//! Where a user's doc comment lands on a `#[cgp_fn]`.
//!
//! The function's doc describes the operation, so it stays on the method, where
//! the function's body lands, and the generated trait carries a line of its own.
//! The generated line is not appended after the user's doc: the method carries one
//! doc line, the user's, and the trait carries one, the generated line. A function
//! with no doc leaves the method with a generated line too, which
//! `doc_comment_on_cgp_fn_without_doc` pins.
//!
//! See cgp-knowledge-base/cgp/implementation/README.md#doc-comments-on-generated-items.

// The doc comment inside the invocation is consumed by the macro, which emits it
// on the generated items, so rustdoc never sees it here.
#![allow(unused_doc_comments)]

use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    /// The user's doc for the operation.
    pub fn greet_fn(&self, #[implicit] name: &str) -> String {
        name.to_owned()
    }

    expand_greet_fn(output) {
        insta::assert_snapshot!(output, @"
        ///The trait for the `greet_fn` operation.
        pub trait GreetFn {
            /// The user's doc for the operation.
            fn greet_fn(&self) -> String;
        }
        /// The user's doc for the operation.
        impl<__Context__> GreetFn for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = String,
            >,
        {
            fn greet_fn(&self) -> String {
                let name: &str = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .as_str();
                name.to_owned()
            }
        }
        ")
    }
}
