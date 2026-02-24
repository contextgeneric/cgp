use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl, ItemStruct, parse_quote, parse2};

use crate::parse::{SimpleType, TypeGenerics};

pub fn cgp_inherit(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let preset: SimpleType = parse2(attr)?;

    let type_generics = TypeGenerics::try_from(&context_struct.generics)?;

    let (delegate_impl, is_provider_impl) =
        derive_delegate_preset(&context_struct.ident, &Some(type_generics), &preset)?;

    Ok(quote! {
        #context_struct

        #delegate_impl

        #is_provider_impl
    })
}

pub fn derive_delegate_preset(
    provider_name: &Ident,
    provider_generics: &Option<TypeGenerics>,
    preset: &SimpleType,
) -> syn::Result<(ItemImpl, ItemImpl)> {
    let preset_name = &preset.name;
    let preset_generics = &preset.generics;

    let provider_params = match provider_generics {
        Some(generics) => {
            let params = &generics.generics.params;
            quote! {
                , #params
            }
        }
        None => quote! {},
    };

    let preset_trait_name = quote! {
        #preset_name :: IsPreset
    };

    let preset_provider_name = quote! {
        #preset_name :: Components #preset_generics
    };

    let delegate_impl: ItemImpl = parse_quote! {
        impl< __Name__ #provider_params >
            DelegateComponent<__Name__>
            for #provider_name #provider_generics
        where
            Self: #preset_trait_name < __Name__ >,
            #preset_provider_name: DelegateComponent<__Name__>,
        {
            type Delegate = <#preset_provider_name as DelegateComponent<__Name__>>::Delegate;
        }
    };

    let is_provider_impl: ItemImpl = parse_quote! {
        impl<__Name__, __Context__, __Params__ #provider_params >
            IsProviderFor<__Name__, __Context__, __Params__>
            for #provider_name #provider_generics
        where
            Self: #preset_trait_name < __Name__ >,
            #preset_provider_name: IsProviderFor<__Name__, __Context__, __Params__>,
        {
        }
    };

    Ok((delegate_impl, is_provider_impl))
}
