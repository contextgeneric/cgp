//! `#[derive(HasField)]` on a struct with a raw-identifier field: the field
//! `r#type` is tagged by its logical name `Symbol!("type")` — the `r#` prefix is
//! stripped — so the field is addressable by the same symbol `Symbol!("type")`
//! produces, while the accessor body still borrows `&self.r#type`.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_has_field.md and
//! cgp-knowledge-base/cgp/reference/macros/symbol.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Context {
        pub r#type: String,
    }

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<4, Chars<'t', Chars<'y', Chars<'p', Chars<'e', Nil>>>>>>
        for Context {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'t', Chars<'y', Chars<'p', Chars<'e', Nil>>>>>,
                >,
            ) -> &Self::Value {
                &self.r#type
            }
        }
        impl HasFieldMut<Symbol<4, Chars<'t', Chars<'y', Chars<'p', Chars<'e', Nil>>>>>>
        for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'t', Chars<'y', Chars<'p', Chars<'e', Nil>>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.r#type
            }
        }
        ")
    }
}

pub trait CheckHasFieldImpls: HasField<Symbol!("type"), Value = String> {}

impl CheckHasFieldImpls for Context {}

#[test]
fn test_raw_ident_field() {
    let context = Context {
        r#type: "widget".to_owned(),
    };

    // The raw-identifier field is reachable by its logical name "type".
    assert_eq!(context.get_field(PhantomData::<Symbol!("type")>), "widget");
}
