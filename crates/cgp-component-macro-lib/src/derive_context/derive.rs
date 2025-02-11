use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, ItemImpl, ItemStruct};

use crate::derive_context::ContextSpec;

pub fn derive_context(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_spec: ContextSpec = syn::parse2(attr)?;

    let context_struct: ItemStruct = syn::parse2(body)?;

    let provider_name = &context_spec.provider_name;

    let provider_struct: ItemStruct = parse_quote!( pub struct #provider_name; );

    let has_components_impl: ItemImpl = derive_has_components(provider_name, &context_struct);

    Ok(quote! {
        #context_struct

        #provider_struct

        #has_components_impl
    })
}

pub fn derive_has_components(provider_name: &Ident, context_struct: &ItemStruct) -> ItemImpl {
    let context_name = &context_struct.ident;

    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    parse_quote! {
        impl #impl_generics HasComponents for #context_name #ty_generics
            #where_clause
        {
            type Components = #provider_name;
        }
    }
}
