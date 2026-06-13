use std::collections::BTreeMap;

use syn::punctuated::Punctuated;
use syn::token::{Comma, Plus};
use syn::visit_mut::{VisitMut, visit_generics_mut};
use syn::{
    GenericArgument, Generics, Ident, PathArguments, PredicateType, Type, TypeParam,
    TypeParamBound, parse_quote,
};

use crate::exports::IsProviderFor;

pub fn replace_provider_in_generics(provider_map: &BTreeMap<Ident, Type>, generics: &mut Generics) {
    let mut visitor = ReplaceProviderVisitor { provider_map };
    visit_generics_mut(&mut visitor, generics);
}

struct ReplaceProviderVisitor<'a> {
    provider_map: &'a BTreeMap<Ident, Type>,
}

impl<'a> VisitMut for ReplaceProviderVisitor<'a> {
    fn visit_type_param_mut(&mut self, type_param: &mut TypeParam) {
        replace_provider_in_type_params(self.provider_map, &mut type_param.bounds);
    }

    fn visit_predicate_type_mut(&mut self, type_predicate: &mut PredicateType) {
        replace_provider_in_type_params(self.provider_map, &mut type_predicate.bounds);
    }
}

fn replace_provider_in_type_params(
    provider_map: &BTreeMap<Ident, Type>,
    type_params: &mut Punctuated<TypeParamBound, Plus>,
) {
    let mut new_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();

    for bound in type_params.iter() {
        if let TypeParamBound::Trait(trait_bound) = bound
            && let Some(segment) = trait_bound.path.segments.last()
            && let Some(component_type) = provider_map.get(&segment.ident).cloned()
            && let PathArguments::AngleBracketed(args) = &segment.arguments
        {
            let mut generics = args.args.iter().map(Clone::clone);
            if let Some(GenericArgument::Type(context_type)) = generics.next() {
                let rest_generics: Punctuated<GenericArgument, Comma> = generics
                    .filter(|arg| {
                        matches!(
                            arg,
                            GenericArgument::Lifetime(_)
                                | GenericArgument::Type(_)
                                | GenericArgument::Const(_)
                        )
                    })
                    .collect();

                let mut new_bound = trait_bound.clone();
                new_bound.path = parse_quote!( #IsProviderFor< #component_type, #context_type, (#rest_generics) > );

                new_bounds.push(TypeParamBound::Trait(new_bound));
            }
        }
    }

    if !new_bounds.is_empty() {
        new_bounds.extend(type_params.clone());
        *type_params = new_bounds;
    }
}
