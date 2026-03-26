use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, GenericParam, TypeParamBound, WherePredicate};

use crate::cgp_fn::{FunctionAttributes, UseTypeSpec};
use crate::cgp_impl::UseProviderSpec;
use crate::parse::SimpleType;

pub fn parse_function_attributes(
    attributes: Vec<Attribute>,
) -> syn::Result<(FunctionAttributes, Vec<Attribute>)> {
    let mut parsed_attributes = FunctionAttributes::default();
    let mut rest_attributes = Vec::new();

    for attribute in attributes.into_iter() {
        if let Some(ident) = attribute.path().get_ident() {
            if ident == "extend" {
                let extend_bound = attribute
                    .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;

                parsed_attributes.extend.extend(extend_bound);
            } else if ident == "extend_where" {
                let where_predicates = attribute
                    .parse_args_with(Punctuated::<WherePredicate, Comma>::parse_terminated)?;

                parsed_attributes.extend_where.extend(where_predicates);
            } else if ident == "uses" {
                let uses =
                    attribute.parse_args_with(Punctuated::<SimpleType, Comma>::parse_terminated)?;

                parsed_attributes.uses.extend(uses);
            } else if ident == "use_type" {
                let use_type = attribute
                    .parse_args_with(Punctuated::<UseTypeSpec, Comma>::parse_terminated)?;

                parsed_attributes.use_type.extend(use_type);
            } else if ident == "use_provider" {
                let use_provider = attribute
                    .parse_args_with(Punctuated::<UseProviderSpec, Comma>::parse_terminated)?;

                parsed_attributes.use_provider.extend(use_provider);
            } else if ident == "impl_generics" {
                let impl_generics = attribute
                    .parse_args_with(Punctuated::<GenericParam, Comma>::parse_terminated)?;

                parsed_attributes.impl_generics.extend(impl_generics);
            } else {
                rest_attributes.push(attribute);
            }
        } else {
            rest_attributes.push(attribute);
        }
    }

    Ok((parsed_attributes, rest_attributes))
}
