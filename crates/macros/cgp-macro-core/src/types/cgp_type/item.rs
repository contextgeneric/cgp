use syn::spanned::Spanned;
use syn::{Error, Item, ItemImpl, ItemTrait, TraitItem, TraitItemType};

use crate::exports::{TypeProvider, UseType, WithProvider};
use crate::parse_internal;
use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::provider_impl::{ItemProviderImpl, ItemProviderImpls};
use crate::visitors::get_bounds_and_replace_self_assoc_type;

pub struct ItemCgpType {
    pub item_component: EvaluatedCgpComponent,
}

impl ItemCgpType {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let mut items = self.item_component.to_items()?;

        let item_impls = self.to_item_provider_impls()?.to_item_impls()?;

        items.extend(item_impls.into_iter().map(Item::from));

        Ok(items)
    }

    pub fn to_trait_item_type(&self) -> syn::Result<TraitItemType> {
        extract_item_type_from_trait(&self.item_component.consumer_trait)
    }

    pub fn to_item_provider_impls(&self) -> syn::Result<ItemProviderImpls> {
        let context_name = &self.item_component.args.context_ident;
        let component_type = self.item_component.args.component_name.to_type();

        let provider_trait = &self.item_component.provider_trait;
        let provider_trait_name = &provider_trait.ident;

        let item_type = self.to_trait_item_type()?;

        let type_name = &item_type.ident;

        let type_bounds = get_bounds_and_replace_self_assoc_type(&item_type);

        let mut generics = provider_trait.generics.clone();
        generics.params.insert(0, parse_internal!(#type_name));

        if !type_bounds.is_empty() {
            generics
                .make_where_clause()
                .predicates
                .push(parse_internal! {
                        #type_name: #type_bounds
                });
        }

        let (_, type_generics, _) = provider_trait.generics.split_for_impl();
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let use_type_impl: ItemImpl = parse_internal! {
            impl #impl_generics
                #provider_trait_name #type_generics
                for #UseType< #type_name >
            #where_clause
            {
                type #type_name = #type_name;
            }
        };

        let use_type_provider = ItemProviderImpl {
            component_type: component_type.clone(),
            item_impl: use_type_impl,
        };

        generics.params.insert(0, parse_internal!(__Provider__));
        generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                __Provider__: #TypeProvider< #context_name, #component_type, Type = #type_name >
            });

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let with_provider_impl: ItemImpl = parse_internal! {
            impl #impl_generics
                #provider_trait_name #type_generics
                for #WithProvider< __Provider__ >
            #where_clause
            {
                type #type_name = #type_name;
            }
        };

        let with_provider_provider = ItemProviderImpl {
            component_type: component_type.clone(),
            item_impl: with_provider_impl,
        };

        Ok(ItemProviderImpls {
            items: vec![use_type_provider, with_provider_provider],
        })
    }
}

pub fn extract_item_type_from_trait(consumer_trait: &ItemTrait) -> syn::Result<TraitItemType> {
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

            Ok(item_type.clone())
        }
        _ => Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        )),
    }
}
