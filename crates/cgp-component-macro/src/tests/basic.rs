use crate::helper::derive::derive_component;
use quote::quote;

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
