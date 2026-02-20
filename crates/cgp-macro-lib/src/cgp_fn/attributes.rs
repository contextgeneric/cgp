use core::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, TypeParamBound, WherePredicate};

use crate::cgp_fn::{FunctionAttributes, UseTypeSpec};
use crate::parse::SimpleType;

pub fn parse_function_attributes(
    attributes: &mut Vec<Attribute>,
) -> syn::Result<FunctionAttributes> {
    let mut parsed_attributes = FunctionAttributes::default();

    let in_attributes = mem::take(attributes);

    for attribute in in_attributes.into_iter() {
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
            } else {
                attributes.push(attribute);
            }
        } else {
            attributes.push(attribute);
        }
    }

    Ok(parsed_attributes)
}
