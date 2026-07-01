//! Chained field access where the outer struct borrows the inner one
//! (`inner: &'a Inner`): the derive lifts the outer lifetime onto its impls,
//! and `ChainGetters` still resolves `outer.inner.name` across the borrow.
//!
//! See docs/reference/derives/derive_has_field.md.

use core::marker::PhantomData;

use cgp::core::field::impls::ChainGetters;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Outer<'a> {
        pub inner: &'a Inner,
    }

    expand_outer(output) {
        insta::assert_snapshot!(output, @"
        impl<
            'a,
        > HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
        for Outer<'a> {
            type Value = &'a Inner;
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
    pub struct Inner {
        pub name: String,
    }

    expand_inner(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for Inner {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                >,
            ) -> &Self::Value {
                &self.name
            }
        }
        impl HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
        for Inner {
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
fn test_chained_getter_with_outer_life() {
    let context = Outer {
        inner: &Inner {
            name: "test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);
    assert_eq!(name, "test");
}
