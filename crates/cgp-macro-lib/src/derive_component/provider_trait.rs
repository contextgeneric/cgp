use alloc::vec::Vec;

use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::{Ident, ItemTrait, TraitItem, TypeParamBound, parse_quote, parse2};

use crate::parse::parse_is_provider_params;
use crate::replace_self::{
    ReplaceSelfTypeVisitor, replace_self_receiver_in_signature, replace_self_value_in_block,
    to_snake_case_ident,
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

    let mut replace_self_type_visitor = ReplaceSelfTypeVisitor {
        replaced_type: &parse_quote!(#context_type),
        skip_assoc_types: &local_assoc_types,
    };

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    {
        let mut context_constraints = provider_trait.supertraits.clone();

        for constraint in &mut context_constraints {
            replace_self_type_visitor.visit_type_param_bound_mut(constraint);
        }

        let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

        let provider_supertrait: TypeParamBound = parse2(quote!(
            IsProviderFor< #component_name < #component_params >, #context_type, ( #is_provider_params ) >
        ))?;

        provider_trait.supertraits = Punctuated::from_iter([provider_supertrait]);

        if !context_constraints.is_empty() {
            match &mut provider_trait.generics.where_clause {
                Some(where_clause) => {
                    replace_self_type_visitor.visit_where_clause_mut(where_clause);
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
        let context_ident = to_snake_case_ident(context_type);

        for trait_item in provider_trait.items.iter_mut() {
            replace_self_type_visitor.visit_trait_item_mut(trait_item);

            if let TraitItem::Fn(func) = trait_item {
                replace_self_receiver_in_signature(
                    &mut func.sig,
                    &context_ident,
                    context_type.to_token_stream(),
                );

                if let Some(block) = &mut func.default {
                    replace_self_value_in_block(block, &context_ident);
                }
            }
        }
    }

    Ok(provider_trait)
}
