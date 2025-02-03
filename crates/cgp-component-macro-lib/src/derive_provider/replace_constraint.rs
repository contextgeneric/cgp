use std::collections::BTreeMap;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_quote, GenericArgument, Generics, Ident, PathArguments, Type, TypeParamBound,
    WherePredicate,
};

pub fn replace_provider_in_generics(provider_map: &BTreeMap<Ident, Type>, generics: &mut Generics) {
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
            for bound in type_predicate.bounds.iter_mut() {
                if let TypeParamBound::Trait(trait_bound) = bound {
                    if let Some(segment) = trait_bound.path.segments.last_mut() {
                        if let Some(component_type) = provider_map.get(&segment.ident).cloned() {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                let mut generics = args.args.iter().map(Clone::clone);
                                if let Some(GenericArgument::Type(context_type)) = generics.next() {
                                    let rest_generics: Punctuated<GenericArgument, Comma> =
                                        generics.collect();

                                    trait_bound.path = parse_quote!( IsProviderFor< #component_type, #context_type, (#rest_generics) > );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
