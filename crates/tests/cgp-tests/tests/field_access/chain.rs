//! Chained field access over two owned structs: each struct derives
//! `HasField`, and `ChainGetters<Product![UseField<..>, UseField<..>]>` composes
//! the two per-field impls to read `outer.inner.name` through a single call. The
//! `#[derive(HasField)]` snapshots are the point of the test; the `ChainGetters`
//! wiring is behavioral scaffolding.
//!
//! See docs/reference/derives/derive_has_field.md and
//! docs/reference/traits/has_field.md.

use core::marker::PhantomData;

use cgp::core::field::impls::ChainGetters;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

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

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Outer {
        pub inner: Inner,
    }

    expand_outer(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
        for Outer {
            type Value = Inner;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                >,
            ) -> &Self::Value {
                &self.inner
            }
        }
        impl HasFieldMut<
            Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
        > for Outer {
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

#[test]
fn test_chained_getter() {
    let context = Outer {
        inner: Inner {
            name: "test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);
    assert_eq!(name, "test");
}
