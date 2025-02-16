use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, Generics, Ident};

use crate::delegate_components::ast::ComponentAst;

pub fn derive_impl_has_component_at(
    preset_module_name: &Ident,
    components_struct_name: &Ident,
    components: &Punctuated<ComponentAst, Comma>,
) -> syn::Result<(TokenStream, TokenStream)> {
    let mut i: usize = 0;

    let mut impl_body = TokenStream::new();
    let mut substitution_body = TokenStream::new();

    for component in components {
        let (impl_generics, type_generics, _) = component.component_generics.split_for_impl();

        let type_generics: Generics = parse2(type_generics.to_token_stream())?;

        let impl_generics_params = type_generics.params;

        let component_type = &component.component_type;

        let item_impl = quote! {
            impl #impl_generics HasComponentAt< #i, (#impl_generics_params) >
                for #components_struct_name
            {
                type Component = #component_type;
            }
        };

        let substitution = quote! {
            #impl_generics ComponentAt< #preset_module_name :: #components_struct_name, #i, (#impl_generics_params) >,
        };

        impl_body.extend(item_impl);
        substitution_body.extend(substitution);

        i += 1;
    }

    Ok((impl_body, substitution_body))
}
