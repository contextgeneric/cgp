use quote::quote;
use syn::{parse2, parse_quote, Ident, ItemImpl, ItemStruct, Path};

use crate::parse::TypeGenerics;

pub fn derive_has_components(
    provider_name: &Ident,
    context_struct: &ItemStruct,
) -> syn::Result<ItemImpl> {
    let context_name = &context_struct.ident;

    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    parse2(quote! {
        impl #impl_generics HasProvider for #context_name #ty_generics
            #where_clause
        {
            type Provider = #provider_name;
        }
    })
}

pub fn derive_delegate_preset(
    provider_name: &Ident,
    preset_name: &Ident,
    preset_generics: &Option<TypeGenerics>,
) -> syn::Result<(ItemImpl, ItemImpl)> {
    let preset_trait_name: Path = parse2(quote! {
        #preset_name :: IsPreset
    })?;

    let preset_provider_name: Path = parse2(quote! {
        #preset_name :: Provider
    })?;

    let delegate_impl: ItemImpl = parse_quote! {
        impl<__Name__>
            DelegateComponent<__Name__>
            for #provider_name
        where
            Self: #preset_trait_name < __Name__ >,
        {
            type Delegate = #preset_provider_name #preset_generics ;
        }
    };

    let is_provider_impl: ItemImpl = parse_quote! {
        impl<__Name__, __Context__, __Params__>
            IsProviderFor<__Name__, __Context__, __Params__>
            for #provider_name
        where
            Self: #preset_trait_name < __Name__ >,
            #preset_provider_name #preset_generics: IsProviderFor<__Name__, __Context__, __Params__>,
        {
        }
    };

    Ok((delegate_impl, is_provider_impl))
}
