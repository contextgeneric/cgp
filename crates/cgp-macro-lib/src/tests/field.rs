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
        impl HasField<ι<'b', ι<'a', ι<'r', ε>>>> for Foo {
            type Value = Bar;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'r', ε>>>>,
            ) -> &Self::Value {
                &self.bar
            }
        }

        impl HasFieldMut<ι<'b', ι<'a', ι<'r', ε>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'r', ε>>>>,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }

        impl HasField<ι<'b', ι<'a', ι<'z', ε>>>> for Foo {
            type Value = Baz;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'z', ε>>>>,
            ) -> &Self::Value {
                &self.baz
            }
        }

        impl HasFieldMut<ι<'b', ι<'a', ι<'z', ε>>>> for Foo {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'z', ε>>>>,
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
        impl<FooParamA, FooParamB: Clone> HasField<ι<'b', ι<'a', ι<'r', ε>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Value = Bar<FooParamA>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'r', ε>>>>,
            ) -> &Self::Value {
                &self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<ι<'b', ι<'a', ι<'r', ε>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'r', ε>>>>,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasField<ι<'b', ι<'a', ι<'z', ε>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Value = Baz<String>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'z', ε>>>>,
            ) -> &Self::Value {
                &self.baz
            }
        }

        impl<FooParamA, FooParamB: Clone> HasFieldMut<ι<'b', ι<'a', ι<'z', ε>>>>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<ι<'b', ι<'a', ι<'z', ε>>>>,
            ) -> &mut Self::Value {
                &mut self.baz
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}
