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
pub fn test_trait_alias_with_method() {
    let derived = trait_alias(
        quote!(),
        quote! {
            pub trait CanDoFooBar: CanDoFoo + CanDoBar {
                fn foo_bar(&self) {
                    self.foo().bar();
                }
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub trait CanDoFooBar: CanDoFoo + CanDoBar {
            fn foo_bar(&self);
        }

        impl<Context> CanDoFooBar for Context
        where
            Context: CanDoFoo + CanDoBar,
        {
            fn foo_bar(&self) {
                self.foo().bar();
            }
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
pub fn test_trait_alias_with_associated_type_without_constraints() {
    let derived = trait_alias(
        quote!(),
        quote! {
            pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
                type FooBar;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
            type FooBar;
        }

        impl<Context, FooBar> HasFooAtBar for Context
        where
            Context: HasFooAt<Bar, Foo = FooBar>,
        {
            type FooBar = FooBar;
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
pub fn test_trait_alias_with_associated_type_and_constraints() {
    let derived = trait_alias(
        quote!(),
        quote! {
            pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
                type FooBar: Clone;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub trait HasFooAtBar: HasFooAt<Bar, Foo = Self::FooBar> {
            type FooBar: Clone;
        }

        impl<Context, FooBar> HasFooAtBar for Context
        where
            Context: HasFooAt<Bar, Foo = FooBar>,
            FooBar: Clone,
        {
            type FooBar = FooBar;
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
