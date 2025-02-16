use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Ident;

use crate::delegate_components::ast::ComponentAst;

pub fn derive_component_aliases(
    preset_module_name: &Ident,
    components: &Punctuated<ComponentAst, Comma>,
) -> syn::Result<(TokenStream, TokenStream)> {
    let mut impl_body = TokenStream::new();

    let mut substitution_body = TokenStream::new();

    for component in components {
        let (impl_generics, _, _) = component.component_generics.split_for_impl();

        let component_type = &component.component_type;

        let item_impl = quote! {
            pub type #component_type = super :: #component_type ;
        };

        let substitution = quote! {
            #impl_generics #preset_module_name ::components:: #component_type,
        };

        impl_body.extend(item_impl);
        substitution_body.extend(substitution);
    }

    Ok((impl_body, substitution_body))
}
