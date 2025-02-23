use quote::quote;

use crate::derive_getter_component;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_derive_getter_basic() {
    let derived = derive_getter_component(
        quote! {
            provider: PersonFieldsGetter,
        },
        quote! {
            pub trait HasName: HasNameType {
                fn name(&self) -> &Self::Name;
            }
        },
    )
    .unwrap();

    println!("derived: {}", format_token_stream(&derived));
}

#[test]
fn test_derive_getter_with_generics() {
    let derived = derive_getter_component(
        quote! {
            provider: PersonFieldsGetter,
        },
        quote! {
            pub trait HasName<App>
            where
                App: HasNameType,
            {
                fn name(&self) -> &App::Name;
            }
        },
    )
    .unwrap();

    println!("derived: {}", format_token_stream(&derived));
}
