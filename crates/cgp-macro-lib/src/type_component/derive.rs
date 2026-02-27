use alloc::vec::Vec;

use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Error, Generics, ItemImpl, ItemTrait, TraitItem, TraitItemType, Type, parse2};

use crate::derive_provider::derive_is_provider_for;
use crate::parse::ComponentSpec;
use crate::type_component::replace::get_bounds_and_replace_self_assoc_type;

pub fn extract_item_type_from_trait(consumer_trait: &ItemTrait) -> syn::Result<&TraitItemType> {
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

pub fn derive_type_providers(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    item_type: &TraitItemType,
) -> syn::Result<Vec<ItemImpl>> {
    let context_name = &spec.context_type;

    let component_name: Type = {
        let name = &spec.component_name;
        let params = &spec.component_params;
        parse2(quote! { #name < #params > })?
    };

    let provider_trait_name = &provider_trait.ident;

    let (impl_generics, type_generics, where_clause) = provider_trait.generics.split_for_impl();

    let impl_generics_params = parse2::<Generics>(impl_generics.to_token_stream())?.params;

    let predicates = where_clause
        .map(|c| c.predicates.clone())
        .unwrap_or_default();

    let type_name = &item_type.ident;

    let type_bounds = get_bounds_and_replace_self_assoc_type(item_type);

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
        impl< __Provider__, #type_name, #impl_generics_params >
            #provider_trait_name #type_generics
            for WithProvider< __Provider__ >
        where
            __Provider__: TypeProvider< #context_name, #component_name, Type = #type_name >,
            #type_name: #type_bounds,
            #predicates
        {
            type #type_name = #type_name;
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
