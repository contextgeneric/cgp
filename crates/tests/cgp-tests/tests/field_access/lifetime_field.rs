//! `#[derive(HasField)]` on a struct with a lifetime-carrying field: the derive
//! lifts the struct's lifetime onto the generated impls and keeps the field's
//! borrowed type (`&'a str`) as the `HasField::Value`.
//!
//! See docs/reference/derives/derive_has_field.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Context<'a> {
        pub name: &'a str,
    }

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl<'a> HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
        for Context<'a> {
            type Value = &'a str;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                >,
            ) -> &Self::Value {
                &self.name
            }
        }
        impl<'a> HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
        for Context<'a> {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.name
            }
        }
        ")
    }
}

#[test]
fn test_context_with_lifetime_field() {
    let context = Context { name: "test" };

    assert_eq!(context.get_field(PhantomData), &"test");
}
