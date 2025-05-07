use quote::quote;
use syn::{parse2, parse_quote, AngleBracketedGenericArguments, Ident, ItemImpl, ItemStruct};

use crate::parse::TypeGenerics;

pub fn derive_has_components(
    provider_name: &Ident,
    provider_generics: &Option<TypeGenerics>,
    context_struct: &ItemStruct,
) -> syn::Result<ItemImpl> {
    let context_name = &context_struct.ident;

    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    parse2(quote! {
        impl #impl_generics HasCgpProvider for #context_name #ty_generics
            #where_clause
        {
            type CgpProvider = #provider_name #provider_generics;
        }
    })
}

pub fn derive_delegate_preset(
    provider_name: &Ident,
    provider_generics: &Option<TypeGenerics>,
    preset_name: &Ident,
    preset_generics: &Option<AngleBracketedGenericArguments>,
) -> syn::Result<(ItemImpl, ItemImpl)> {
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
        #preset_name :: Provider #preset_generics
    };

    let delegate_impl: ItemImpl = parse_quote! {
        impl< __Name__ #provider_params >
            DelegateComponent<__Name__>
            for #provider_name #provider_generics
        where
            Self: #preset_trait_name < __Name__ >,
        {
            type Delegate = #preset_provider_name;
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
