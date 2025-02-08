use std::collections::BTreeMap;

use syn::punctuated::Punctuated;
use syn::token::{Comma, Plus};
use syn::{
    parse_quote, GenericArgument, GenericParam, Generics, Ident, PathArguments, Type,
    TypeParamBound, WherePredicate,
};

pub fn replace_provider_in_generics(provider_map: &BTreeMap<Ident, Type>, generics: &mut Generics) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            replace_provider_in_type_params(provider_map, &mut type_param.bounds);
        }
    }

    if let Some(where_clause) = &mut generics.where_clause {
        replace_provider_in_where_predicate(provider_map, &mut where_clause.predicates);
    }
}

pub fn replace_provider_in_where_predicate(
    provider_map: &BTreeMap<Ident, Type>,
    predicates: &mut Punctuated<WherePredicate, Comma>,
) {
    for predicate in predicates.iter_mut() {
        if let WherePredicate::Type(type_predicate) = predicate {
            replace_provider_in_type_params(provider_map, &mut type_predicate.bounds);
        }
    }
}

pub fn replace_provider_in_type_params(
    provider_map: &BTreeMap<Ident, Type>,
    type_params: &mut Punctuated<TypeParamBound, Plus>,
) {
    let mut new_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();

    for bound in type_params.iter() {
        if let TypeParamBound::Trait(trait_bound) = bound {
            if let Some(segment) = trait_bound.path.segments.last() {
                if let Some(component_type) = provider_map.get(&segment.ident).cloned() {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        let mut generics = args.args.iter().map(Clone::clone);
                        if let Some(GenericArgument::Type(context_type)) = generics.next() {
                            let rest_generics: Punctuated<GenericArgument, Comma> =
                                generics.collect();

                            let mut new_bound = trait_bound.clone();
                            new_bound.path = parse_quote!( IsProviderFor< #component_type, #context_type, (#rest_generics) > );

                            new_bounds.push(TypeParamBound::Trait(new_bound));
                        }
                    }
                }
            }
        }
    }

    if !new_bounds.is_empty() {
        new_bounds.extend(type_params.clone());
        *type_params = new_bounds;
    }
}
