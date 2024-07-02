use quote::quote;

use crate::derive_component::derive::derive_component;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_basic_derive_component() {
    derive_component(
        quote! { FooComponent, FooProvider<Context> },
        quote! {
            pub trait HasFoo<Bar> {
                type Foo;

                fn foo(&self) -> Self::Foo;
            }
        },
    );
}

#[test]
fn test_derive_component_with_const_generic() {
    let derived = derive_component(
        quote! { FooComponent, FooProvider<Context> },
        quote! {
            pub trait HasFoo<const BAR: usize> {
                type Foo;

                fn foo(&self) -> Self::Foo;
            }
        },
    );

    let expected = quote! {
        pub trait HasFoo<const BAR: usize> {
            type Foo;

            fn foo(&self) -> Self::Foo;
        }

        pub struct FooComponent;

        pub trait FooProvider<Context, const BAR: usize>: Async {
            type Foo;

            fn foo(context: &Context) -> Self::Foo;
        }

        impl<Context, const BAR: usize> HasFoo<BAR> for Context
        where
            Context: HasComponents,
            Context::Components: FooProvider<Context, BAR>,
        {
            type Foo = <Context::Components as FooProvider<Context, BAR>>::Foo;

            fn foo(&self) -> Self::Foo {
                Context::Components::foo(self)
            }
        }

        impl<Component, Context, const BAR: usize> FooProvider<Context, BAR> for Component
        where
            Component: DelegateComponent<FooComponent>,
            Component::Delegate: FooProvider<Context, BAR>,
        {
            type Foo = <Component::Delegate as FooProvider<Context, BAR>>::Foo;

            fn foo(context: &Context) -> Self::Foo {
                Component::Delegate::foo(context)
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}
