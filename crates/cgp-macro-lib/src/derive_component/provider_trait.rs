use alloc::vec::Vec;

use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, ItemTrait, TraitItem, TypeParamBound, parse2};

use crate::parse::parse_is_provider_params;
use crate::replace_self::{
    iter_parse_and_replace_self_type, parse_and_replace_self_type,
    replace_self_receiver_in_signature, replace_self_var, to_snake_case_ident,
};

pub fn derive_provider_trait(
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemTrait> {
    let mut provider_trait = consumer_trait.clone();

    provider_trait.ident = provider_name.clone();

    // Add generic parameter `Context` to the front of generics
    {
        provider_trait
            .generics
            .params
            .insert(0, parse2(quote!(#context_type))?);
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
    {
        let context_constraints = iter_parse_and_replace_self_type(
            provider_trait.supertraits.clone(),
            context_type,
            &local_assoc_types,
        )?;

        let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

        let provider_supertrait: TypeParamBound = parse2(quote!(
            IsProviderFor< #component_name < #component_params >, #context_type, ( #is_provider_params ) >
        ))?;

        provider_trait.supertraits = Punctuated::from_iter([provider_supertrait]);

        if !context_constraints.is_empty() {
            match &mut provider_trait.generics.where_clause {
                Some(where_clause) => {
                    let mut predicates = iter_parse_and_replace_self_type(
                        where_clause.predicates.clone(),
                        context_type,
                        &local_assoc_types,
                    )?;

                    predicates.push(parse2(quote! {
                        #context_type : #context_constraints
                    })?);

                    where_clause.predicates = predicates;
                }
                _ => {
                    provider_trait.generics.where_clause = Some(parse2(quote! {
                        where #context_type : #context_constraints
                    })?);
                }
            }
        }
    }

    // Replace self type and argument into context type argument
    {
        let context_var = to_snake_case_ident(context_type);

        for item in provider_trait.items.iter_mut() {
            let mut replaced_item =
                parse_and_replace_self_type(item, context_type, &local_assoc_types)?;

            if let TraitItem::Fn(func) = &mut replaced_item {
                replace_self_receiver_in_signature(
                    &mut func.sig,
                    &context_var,
                    context_type.to_token_stream(),
                );

                if let Some(block) = &mut func.default {
                    let replaced = replace_self_var(block.to_token_stream(), &context_var);
                    *block = parse2(replaced)?;
                }
            }

            *item = replaced_item;
        }
    }

    Ok(provider_trait)
}
