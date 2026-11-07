//! `#[cgp_fn]` with a `&mut self` receiver whose implicit arguments are all
//! immutable.
//!
//! The access mode of an implicit argument follows the argument's own type, not
//! the receiver's, so these `&str` reads go through `HasField`/`get_field` even
//! though the receiver is `&mut self` — contrast `cgp_fn_mutable.rs`, where a
//! `&mut` argument reads through `HasFieldMut`/`get_field_mut`. It also shows that
//! any number of immutable implicits may share a `&mut self` receiver (only a
//! `&mut` implicit is restricted to being the sole implicit argument).
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_fn.md and
//! cgp-knowledge-base/cgp/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn format_label(&mut self, #[implicit] prefix: &str, #[implicit] suffix: &str) -> String {
        format!("{prefix}-{suffix}")
    }

    expand_format_label(output) {
        insta::assert_snapshot!(output, @r#"
        ///The trait for the `format_label` operation.
        pub trait FormatLabel {
            ///The `format_label` operation.
            fn format_label(&mut self) -> String;
        }
        impl<__Context__> FormatLabel for __Context__
        where
            Self: HasField<
                    Symbol<
                        6,
                        Chars<
                            'p',
                            Chars<'r', Chars<'e', Chars<'f', Chars<'i', Chars<'x', Nil>>>>>,
                        >,
                    >,
                    Value: ::core::convert::AsRef<str> + 'static,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'u', Chars<'f', Chars<'f', Chars<'i', Chars<'x', Nil>>>>>,
                        >,
                    >,
                    Value: ::core::convert::AsRef<str> + 'static,
                >,
        {
            fn format_label(&mut self) -> String {
                let prefix: &str = ::core::convert::AsRef::as_ref(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'p',
                                    Chars<
                                        'r',
                                        Chars<'e', Chars<'f', Chars<'i', Chars<'x', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    ),
                );
                let suffix: &str = ::core::convert::AsRef::as_ref(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    's',
                                    Chars<
                                        'u',
                                        Chars<'f', Chars<'f', Chars<'i', Chars<'x', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    ),
                );
                format!("{prefix}-{suffix}")
            }
        }
        "#)
    }
}

#[derive(HasField)]
pub struct Labels {
    pub prefix: String,
    pub suffix: String,
}

pub trait CheckLabels: FormatLabel {}
impl CheckLabels for Labels {}
