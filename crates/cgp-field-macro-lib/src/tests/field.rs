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
        impl HasField<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>> for Foo {
            type Field = Bar;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>,
            ) -> &Self::Field {
                &self.bar
            }
        }

        impl HasFieldMut<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>,
            ) -> &mut Self::Field {
                &mut self.bar
            }
        }

        impl HasField<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>> for Foo {
            type Field = Baz;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>,
            ) -> &Self::Field {
                &self.baz
            }
        }

        impl HasFieldMut<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>,
            ) -> &mut Self::Field {
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
        impl<FooParamA, FooParamB: Clone> HasField<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Field = Bar<FooParamA>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>,
            ) -> &Self::Field {
                &self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'r'> , Nil>>>>,
            ) -> &mut Self::Field {
                &mut self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasField<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Field = Baz<String>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>,
            ) -> &Self::Field {
                &self.baz
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Cons<Char<'b'>, Cons<Char<'a'>, Cons<Char<'z'> , Nil>>>>,
            ) -> &mut Self::Field {
                &mut self.baz
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}
