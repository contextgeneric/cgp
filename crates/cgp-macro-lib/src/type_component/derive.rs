use alloc::format;
use alloc::vec::Vec;

use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse2, Error, Generics, Ident, ItemImpl, ItemTrait, ItemType, TraitItem, TraitItemType, Type,
};

use crate::derive_provider::derive_is_provider_for;
use crate::parse::ComponentSpec;

pub fn extract_item_type(consumer_trait: &ItemTrait) -> syn::Result<&TraitItemType> {
    if consumer_trait.items.len() != 1 {
        return Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        ));
    }

    match consumer_trait.items.first() {
        Some(TraitItem::Type(item_type)) => {
            if !item_type.generics.params.is_empty() || item_type.generics.where_clause.is_some() {
                return Err(Error::new(
                    consumer_trait.span(),
                    "generic associated type and where clause are not supported",
                ));
            }

            Ok(item_type)
        }
        _ => Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        )),
    }
}

pub fn derive_type_alias(
    consumer_trait: &ItemTrait,
    context_name: &Ident,
    item_type: &TraitItemType,
) -> syn::Result<ItemType> {
    let consumer_trait_name = &consumer_trait.ident;

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();

    let type_generics: Generics = parse2(type_generics.to_token_stream())?;

    let type_generics_params = &type_generics.params;

    let type_name = &item_type.ident;
    let alias_name = Ident::new(&format!("{type_name}Of"), type_name.span());

    let alias_type: ItemType = parse2(quote! {
        pub type #alias_name < #context_name, #type_generics_params > =
            < #context_name as #consumer_trait_name #type_generics > :: #type_name ;
    })?;

    Ok(alias_type)
}

pub fn derive_type_providers(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    item_type: &TraitItemType,
) -> syn::Result<Vec<ItemImpl>> {
    let context_name = &spec.context_type;

    let component_name = {
        let name = &spec.component_name;
        let params = &spec.component_params;
        parse2::<Type>(quote! { #name < #params > })
    }?;

    let provider_trait_name = &provider_trait.ident;

    let (impl_generics, type_generics, where_clause) = provider_trait.generics.split_for_impl();

    let impl_generics_params = parse2::<Generics>(impl_generics.to_token_stream())?.params;

    let predicates = where_clause
        .map(|c| c.predicates.clone())
        .unwrap_or_default();

    let type_name = &item_type.ident;

    let type_bounds = &item_type.bounds;

    let use_type_impl: ItemImpl = parse2(quote! {
        impl< #type_name, #impl_generics_params >
            #provider_trait_name #type_generics
            for UseType< #type_name >
        where
            #type_name: #type_bounds,
            #predicates
        {
            type #type_name = #type_name;
        }
    })?;

    let use_type_is_provider_impl = derive_is_provider_for(&component_name, &use_type_impl)?;

    let with_provider_impl: ItemImpl = parse2(quote! {
        impl< __Provider__, #impl_generics_params >
            #provider_trait_name #type_generics
            for WithProvider< __Provider__ >
        where
            __Provider__: ProvideType< #context_name, #component_name >,
            __Provider__::Type: #type_bounds,
            #predicates
        {
            type #type_name = __Provider__::Type;
        }
    })?;

    let with_provider_is_provider_impl =
        derive_is_provider_for(&component_name, &with_provider_impl)?;

    Ok(vec![
        use_type_impl,
        use_type_is_provider_impl,
        with_provider_impl,
        with_provider_is_provider_impl,
    ])
}
