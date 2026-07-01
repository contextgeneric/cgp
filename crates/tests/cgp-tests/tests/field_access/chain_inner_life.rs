//! Chained field access where the inner struct itself carries a lifetime
//! (`Inner<'a>` holding `&'a String`): both derives lift the lifetime onto their
//! impls, and `ChainGetters` composes them to read the borrowed name.
//!
//! See docs/reference/derives/derive_has_field.md.

use core::marker::PhantomData;

use cgp::core::field::impls::ChainGetters;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Outer<'a> {
        pub inner: Inner<'a>,
    }

    expand_outer(output) {
        insta::assert_snapshot!(output, @"
        impl<
            'a,
        > HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
        for Outer<'a> {
            type Value = Inner<'a>;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                >,
            ) -> &Self::Value {
                &self.inner
            }
        }
        impl<
            'a,
        > HasFieldMut<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
        for Outer<'a> {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.inner
            }
        }
        ")
    }
}

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Inner<'a> {
        pub name: &'a String,
    }

    expand_inner(output) {
        insta::assert_snapshot!(output, @"
        impl<'a> HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
        for Inner<'a> {
            type Value = &'a String;
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
        for Inner<'a> {
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
fn test_chained_getter_with_inner_life() {
    let context = Outer {
        inner: Inner {
            name: &"test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);

    assert_eq!(name, "test");
}
