use crate::helper::derive::derive_component;
use quote::quote;

#[test]
fn test_derive_component_with_const_generic() {
    derive_component(
        quote! { FooComponent, FooProvider<Context> },
        quote! {
            pub trait HasFoo<const BAR: usize> {
                type Foo;

                fn foo(&self) -> Self::Foo;
            }
        }
    );
}