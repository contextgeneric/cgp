use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemImpl, ItemStruct};

use crate::derive_context::{derive_delegate_preset, derive_has_components};
use crate::parse::ContextSpec;

pub fn cgp_context(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_spec: ContextSpec = syn::parse2(attr)?;

    let context_struct: ItemStruct = syn::parse2(body)?;

    let provider_name = &context_spec.provider_name;

    let provider_struct: ItemStruct = parse2(quote!( pub struct #provider_name; ))?;

    let has_components_impl: ItemImpl = derive_has_components(provider_name, &context_struct)?;

    let base_derived = quote! {
        #context_struct

        #provider_struct

        #has_components_impl
    };

    match &context_spec.preset {
        Some(preset) => {
            let (delegate_impl, is_provider_impl) =
                derive_delegate_preset(provider_name, &preset.name, &preset.generics)?;

            Ok(quote! {
                #base_derived

                #delegate_impl

                #is_provider_impl
            })
        }
        _ => Ok(base_derived),
    }
}
