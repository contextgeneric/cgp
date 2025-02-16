use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Ident;

use crate::delegate_components::ast::ComponentAst;

pub fn derive_component_aliases(
    preset_module_name: &Ident,
    components: &Punctuated<ComponentAst, Comma>,
) -> syn::Result<(TokenStream, TokenStream)> {
    let mut i: usize = 0;

    let mut impl_body = TokenStream::new();

    let mut substitution_body = TokenStream::new();

    for component in components {
        let (impl_generics, type_generics, _) = component.component_generics.split_for_impl();

        let component_type = &component.component_type;

        let alias_type_name = Ident::new(&format!("Component_{i}"), Span::call_site());

        let item_impl = quote! {
            pub type #alias_type_name #impl_generics = #component_type;
        };

        let substitution = quote! {
            #component_type :
            #impl_generics
            #preset_module_name :: components :: #alias_type_name #type_generics,
        };

        impl_body.extend(item_impl);
        substitution_body.extend(substitution);

        i += 1;
    }

    Ok((impl_body, substitution_body))
}
