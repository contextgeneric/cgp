use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, GenericParam, TypeParamBound, WherePredicate};

use crate::types::attributes::{
    UseProviderAttribute, UseProviderAttributes, UseTypeAttribute, UseTypeAttributes,
};
use crate::types::ident::PathWithTypeArgs;

#[derive(Default)]
pub struct FunctionAttributes {
    pub extend: Vec<TypeParamBound>,
    pub extend_where: Vec<WherePredicate>,
    pub uses: Vec<PathWithTypeArgs>,
    pub use_type: UseTypeAttributes,
    pub use_provider: UseProviderAttributes,
    pub impl_generics: Vec<GenericParam>,
    pub raw_attributes: Vec<Attribute>,
}

impl FunctionAttributes {
    pub fn parse(attributes: Vec<Attribute>) -> syn::Result<Self> {
        let mut parsed_attributes = FunctionAttributes::default();

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
                    let uses = attribute
                        .parse_args_with(Punctuated::<PathWithTypeArgs, Comma>::parse_terminated)?;

                    parsed_attributes.uses.extend(uses);
                } else if ident == "use_type" {
                    let use_type = attribute
                        .parse_args_with(Punctuated::<UseTypeAttribute, Comma>::parse_terminated)?;

                    parsed_attributes.use_type.attributes.extend(use_type);
                } else if ident == "use_provider" {
                    let use_provider = attribute.parse_args_with(
                        Punctuated::<UseProviderAttribute, Comma>::parse_terminated,
                    )?;

                    parsed_attributes
                        .use_provider
                        .attributes
                        .extend(use_provider);
                } else if ident == "impl_generics" {
                    let impl_generics = attribute
                        .parse_args_with(Punctuated::<GenericParam, Comma>::parse_terminated)?;

                    parsed_attributes.impl_generics.extend(impl_generics);
                } else {
                    parsed_attributes.raw_attributes.push(attribute);
                }
            } else {
                parsed_attributes.raw_attributes.push(attribute);
            }
        }

        Ok(parsed_attributes)
    }
}
