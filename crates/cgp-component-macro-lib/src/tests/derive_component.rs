use quote::quote;

use crate::cgp_component;
use crate::tests::helper::equal::assert_equal_token_stream;

#[test]
fn test_basic_derive_component() {
    cgp_component(
        quote! {
            name: FooComponent,
            provider: FooProvider,
        },
        quote! {
            pub trait HasFoo<Bar> {
                type Foo;

                fn foo(&self) -> Self::Foo;
            }
        },
    )
    .unwrap();
}

#[test]
fn test_derive_component_with_const_generic() {
    let derived = cgp_component(
        quote! {
            name: FooComponent,
            provider: FooProvider,
        },
        quote! {
            pub trait HasFoo<const BAR: usize> {
                type Foo;

                fn foo(&self) -> Self::Foo;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct FooComponent;

        pub trait HasFoo<const BAR: usize> {
            type Foo;

            fn foo(&self) -> Self::Foo;
        }

        pub trait FooProvider<Context, const BAR: usize>: IsProviderFor<FooComponent, Context, (BAR)> {
            type Foo;

            fn foo(context: &Context) -> Self::Foo;
        }

        impl<Context, const BAR: usize> HasFoo<BAR> for Context
        where
            Context: HasProvider,
            Context::Provider: FooProvider<Context, BAR>,
        {
            type Foo = <Context::Provider as FooProvider<Context, BAR>>::Foo;

            fn foo(&self) -> Self::Foo {
                Context::Provider::foo(self)
            }
        }

        impl<Component, Context, const BAR: usize> FooProvider<Context, BAR> for Component
        where
            Component: DelegateComponent<FooComponent> + IsProviderFor<FooComponent, Context, (BAR)>,
            Component::Delegate: FooProvider<Context, BAR>,
        {
            type Foo = <Component::Delegate as FooProvider<Context, BAR>>::Foo;

            fn foo(context: &Context) -> Self::Foo {
                Component::Delegate::foo(context)
            }
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
