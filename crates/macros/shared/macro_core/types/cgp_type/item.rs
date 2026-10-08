use syn::spanned::Spanned;
use syn::{Error, Generics, ImplItem, Item, ItemImpl, ItemTrait, TraitItem, TraitItemType};

use crate::macro_core::exports::{TypeProvider, UseType, WithProvider};
use crate::macro_core::parse_internal;
use crate::macro_core::types::cgp_component::EvaluatedCgpComponent;
use crate::macro_core::types::provider_impl::{ItemProviderImpl, ItemProviderImpls};
use crate::macro_core::visitors::get_bounds_and_replace_self_assoc_type;

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
        Ok(ItemProviderImpls {
            items: vec![self.to_use_type_impl()?, self.to_with_provider_impl()?],
        })
    }

    pub fn to_use_type_impl(&self) -> syn::Result<ItemProviderImpl> {
        let (component_type, provider_trait_name, type_generics, type_name, generics, type_item) =
            self.prepare_type_provider()?;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let use_type_impl: ItemImpl = parse_internal! {
            impl #impl_generics
                #provider_trait_name #type_generics
                for #UseType< #type_name >
            #where_clause
            {
                #type_item
            }
        };

        Ok(ItemProviderImpl {
            component_type,
            item_impl: use_type_impl,
        })
    }

    /// The `WithProvider<Provider>` bridge: `Provider: TypeProvider<Context, Component, Type = Name>`.
    pub fn to_with_provider_impl(&self) -> syn::Result<ItemProviderImpl> {
        let context_name = &self.item_component.args.context_ident;
        let (
            component_type,
            provider_trait_name,
            type_generics,
            type_name,
            mut generics,
            type_item,
        ) = self.prepare_type_provider()?;

        // Insert the provider as the new leading generic (position 0, lifetime-safe
        // via `syn::Generics::to_tokens`). See cgp-knowledge-base-fork/cgp/implementation/README.md,
        // "Generic-parameter insertion and lifetime ordering".
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
                #type_item
            }
        };

        Ok(ItemProviderImpl {
            component_type,
            item_impl: with_provider_impl,
        })
    }

    fn prepare_type_provider(
        &self,
    ) -> syn::Result<(
        syn::Type,
        syn::Ident,
        syn::TypeGenerics<'_>,
        syn::Ident,
        Generics,
        ImplItem,
    )> {
        let component_type = self.item_component.args.component_name.to_type();
        let provider_trait = &self.item_component.provider_trait;
        let provider_trait_name = provider_trait.ident.clone();
        let item_type = self.to_trait_item_type()?;
        let type_name = item_type.ident.clone();

        let mut generics = provider_trait.generics.clone();
        // The abstract type leads the impl generics. Position 0 is safe with a
        // lifetime present because `syn::Generics::to_tokens` emits lifetimes first.
        // See cgp-knowledge-base-fork/cgp/implementation/README.md, "Generic-parameter insertion and
        // lifetime ordering".
        let type_item = lift_assoc_type(&mut generics, &item_type, true)?;
        let (_, type_generics, _) = provider_trait.generics.split_for_impl();

        Ok((
            component_type,
            provider_trait_name,
            type_generics,
            type_name,
            generics,
            type_item,
        ))
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
            validate_assoc_type_item(item_type)?;
            Ok(item_type.clone())
        }
        _ => Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        )),
    }
}

/// Reject a generic associated type or a where-clause on the item. `#[cgp_type]`
/// and the getter macros share this check.
pub fn validate_assoc_type_item(item_type: &TraitItemType) -> syn::Result<()> {
    if !item_type.generics.params.is_empty() || item_type.generics.where_clause.is_some() {
        return Err(Error::new(
            item_type.span(),
            "generic associated type and where clause are not supported",
        ));
    }

    Ok(())
}

/// Lift an associated type into an impl generic, its bound, and a
/// `type Name = Name` item — the associated-type path `#[cgp_type]` emits.
///
/// `leading` inserts the parameter at position 0 (safe alongside a lifetime;
/// `syn` emits lifetimes first). Getters append it instead, after the context
/// parameter already on the impl.
pub fn lift_assoc_type(
    generics: &mut Generics,
    item_type: &TraitItemType,
    leading: bool,
) -> syn::Result<ImplItem> {
    validate_assoc_type_item(item_type)?;

    let type_name = &item_type.ident;
    let param = parse_internal!(#type_name);

    if leading {
        generics.params.insert(0, param);
    } else {
        generics.params.push(param);
    }

    let type_bounds = get_bounds_and_replace_self_assoc_type(item_type);

    if !type_bounds.is_empty() {
        generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                #type_name: #type_bounds
            });
    }

    Ok(parse_internal! {
        type #type_name = #type_name;
    })
}
