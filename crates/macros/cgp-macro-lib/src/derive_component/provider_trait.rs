use alloc::vec::Vec;

use cgp_macro_core::functions::to_snake_case_ident;
use cgp_macro_core::visitors::{
    ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor, ReplaceSelfValueVisitor,
};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::{Ident, ItemTrait, TraitItem, Type, TypeParamBound, parse_quote, parse2};

use crate::parse::parse_is_provider_params;

pub fn derive_provider_trait(
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type_ident: &Ident,
) -> syn::Result<ItemTrait> {
    let mut provider_trait = consumer_trait.clone();

    provider_trait.ident = provider_name.clone();

    let context_type: Type = parse_quote!(#context_type_ident);

    // Add generic parameter `Context` to the front of generics
    {
        provider_trait
            .generics
            .params
            .insert(0, parse2(quote!(#context_type_ident))?);
    }

    let local_assoc_types: Vec<Ident> = provider_trait
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Type(assoc_type) = item {
                Some(assoc_type.ident.clone())
            } else {
                None
            }
        })
        .collect();

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    if !provider_trait.supertraits.is_empty() {
        let supertraits = &provider_trait.supertraits;

        provider_trait
            .generics
            .make_where_clause()
            .predicates
            .push(parse_quote! {
                #context_type_ident : #supertraits
            });
    }

    let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

    let provider_supertrait: TypeParamBound = parse2(quote!(
        IsProviderFor< #component_name < #component_params >, #context_type_ident, ( #is_provider_params ) >
    ))?;

    provider_trait.supertraits = Punctuated::from_iter([provider_supertrait]);

    let context_value_ident = to_snake_case_ident(context_type_ident);

    ReplaceSelfTypeVisitor {
        replaced_type: &context_type,
        skip_assoc_types: &local_assoc_types,
    }
    .visit_item_trait_mut(&mut provider_trait);

    ReplaceSelfReceiverVisitor {
        replaced_ident: &context_value_ident,
        replaced_type: &context_type,
    }
    .visit_item_trait_mut(&mut provider_trait);

    ReplaceSelfValueVisitor {
        replaced_ident: &context_value_ident,
    }
    .visit_item_trait_mut(&mut provider_trait);

    Ok(provider_trait)
}
