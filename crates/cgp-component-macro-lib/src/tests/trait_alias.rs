use quote::quote;

use crate::tests::helper::equal::assert_equal_token_stream;
use crate::trait_alias;

#[test]
pub fn test_basic_trait_alias() {
    let derived = trait_alias(
        quote!(),
        quote! {
            pub trait CanDoFooBar: CanDoFoo + CanDoBar {}
        },
    )
    .unwrap();

    let expected = quote! {
        pub trait CanDoFooBar: CanDoFoo + CanDoBar {}

        impl<Context> CanDoFooBar for Context
        where
            Context: CanDoFoo + CanDoBar,
        {}
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
pub fn test_trait_alias_with_associated_type() {
    let derived = trait_alias(
        quote!(),
        quote! {
            pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
                type FooBar = Self::Foo;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
            type FooBar;
        }

        impl<Context> HasFooAtBar for Context
        where
            Context: HasFooAt<Bar>,
        {
            type FooBar = Self::Foo;
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
