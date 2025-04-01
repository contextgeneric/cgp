use quote::quote;

use crate::field::derive_fields;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_basic_derive_fields() {
    let derived = derive_fields(quote! {
        pub struct Foo {
            pub bar: Bar,
            pub baz: Baz,
        }
    });

    let expected = quote! {
        impl HasField<Char<'b', Char<'a', Char<'r', Nil>>>> for Foo {
            type Value = Bar;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'r', Nil>>>>,
            ) -> &Self::Value {
                &self.bar
            }
        }

        impl HasFieldMut<Char<'b', Char<'a', Char<'r', Nil>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'r', Nil>>>>,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }

        impl HasField<Char<'b', Char<'a', Char<'z', Nil>>>> for Foo {
            type Value = Baz;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'z', Nil>>>>,
            ) -> &Self::Value {
                &self.baz
            }
        }

        impl HasFieldMut<Char<'b', Char<'a', Char<'z', Nil>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'z', Nil>>>>,
            ) -> &mut Self::Value {
                &mut self.baz
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_generic_derive_fields() {
    let derived = derive_fields(quote! {
        pub struct Foo<FooParamA, FooParamB: Clone>
        where
            FooParamA: Eq,
        {
            pub bar: Bar<FooParamA>,
            pub baz: Baz<String>,
        }
    });

    let expected = quote! {
        impl<FooParamA, FooParamB: Clone> HasField<Char<'b', Char<'a', Char<'r', Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Value = Bar<FooParamA>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'r', Nil>>>>,
            ) -> &Self::Value {
                &self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<Char<'b', Char<'a', Char<'r', Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'r', Nil>>>>,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasField<Char<'b', Char<'a', Char<'z', Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Value = Baz<String>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'z', Nil>>>>,
            ) -> &Self::Value {
                &self.baz
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<Char<'b', Char<'a', Char<'z', Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Char<'b', Char<'a', Char<'z', Nil>>>>,
            ) -> &mut Self::Value {
                &mut self.baz
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}
