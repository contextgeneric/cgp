use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemImpl, ItemStruct};

use crate::derive_context::{derive_delegate_preset, derive_has_components};
use crate::parse::ContextSpec;

pub fn cgp_context(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = syn::parse2(body)?;

    let context_spec: ContextSpec = if !attr.is_empty() {
        syn::parse2(attr)?
    } else {
        let provider_name = Ident::new(
            &format!("{}Components", context_struct.ident),
            context_struct.ident.span(),
        );

        ContextSpec {
            provider_name,
            provider_generics: None,
            preset: None,
        }
    };

    let provider_name = &context_spec.provider_name;
    let provider_generics = &context_spec.provider_generics;

    let provider_phantom = match provider_generics {
        Some(generics) => {
            let params = &generics.generics.params;
            quote! { ( ::core::marker::PhantomData<( #params )> ) }
        }
        None => quote! {},
    };

    let provider_struct: ItemStruct =
        parse2(quote!( pub struct #provider_name #provider_generics #provider_phantom; ))?;

    let has_components_impl: ItemImpl =
        derive_has_components(provider_name, provider_generics, &context_struct)?;

    let base_derived = quote! {
        #context_struct

        #provider_struct

        #has_components_impl
    };

    match &context_spec.preset {
        Some((preset_path, preset_generics)) => {
            let (delegate_impl, is_provider_impl) = derive_delegate_preset(
                provider_name,
                provider_generics,
                preset_path,
                preset_generics,
            )?;

            Ok(quote! {
                #base_derived

                #delegate_impl

                #is_provider_impl
            })
        }
        _ => Ok(base_derived),
    }
}
