use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, GenericParam, TypeParamBound, WherePredicate};

use crate::types::attributes::{
    UseProviderAttribute, UseProviderAttributes, UseTypeAttribute, UseTypeAttributes,
};

/// The modifier attributes a `#[cgp_fn]` or `#[cgp_impl]` function carries.
#[derive(Default)]
pub struct FunctionAttributes {
    /// The `#[extend]` supertraits.
    pub extend: Vec<TypeParamBound>,
    /// The `#[extend_where]` predicates.
    pub extend_where: Vec<WherePredicate>,
    /// The `#[uses]` trait bounds.
    pub uses: Vec<TypeParamBound>,
    /// The `#[use_type]` imports.
    pub use_type: UseTypeAttributes,
    /// The `#[use_provider]` imports.
    pub use_provider: UseProviderAttributes,
    /// The `#[impl_generics]` parameters.
    pub impl_generics: Vec<GenericParam>,
    /// The attributes the macro passes through.
    pub raw_attributes: Vec<Attribute>,
}

impl FunctionAttributes {
    /// Parses the function's modifier attributes.
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
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;

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
