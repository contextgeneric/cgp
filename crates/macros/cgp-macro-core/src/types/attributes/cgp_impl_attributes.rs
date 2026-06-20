use syn::Attribute;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::types::attributes::{
    DefaultImplAttribute, DefaultImplAttributes, UseProviderAttribute, UseProviderAttributes,
    UseTypeAttribute, UseTypeAttributes, UsesAttributes,
};
use crate::types::ident::PathWithTypeArgs;

#[derive(Default)]
pub struct CgpImplAttributes {
    pub uses: UsesAttributes,
    pub use_type: UseTypeAttributes,
    pub use_provider: UseProviderAttributes,
    pub default_impls: DefaultImplAttributes,
    pub raw_attributes: Vec<Attribute>,
}

impl CgpImplAttributes {
    pub fn parse(attributes: &Vec<Attribute>) -> syn::Result<CgpImplAttributes> {
        let mut parsed_attributes = CgpImplAttributes::default();

        for attribute in attributes {
            if let Some(ident) = attribute.path().get_ident() {
                match ident.to_string().as_ref() {
                    "uses" => {
                        let uses = attribute.parse_args_with(
                            Punctuated::<PathWithTypeArgs, Comma>::parse_terminated,
                        )?;

                        parsed_attributes.uses.imports.extend(uses);
                    }
                    "use_type" => {
                        let use_type = attribute.parse_args_with(
                            Punctuated::<UseTypeAttribute, Comma>::parse_terminated,
                        )?;

                        parsed_attributes.use_type.attributes.extend(use_type);
                    }
                    "use_provider" => {
                        let use_provider = attribute.parse_args_with(
                            Punctuated::<UseProviderAttribute, Comma>::parse_terminated,
                        )?;

                        parsed_attributes
                            .use_provider
                            .attributes
                            .extend(use_provider);
                    }
                    "default_impl" => {
                        let default_impl =
                            attribute.parse_args_with(DefaultImplAttribute::parse)?;

                        parsed_attributes
                            .default_impls
                            .attributes
                            .push(default_impl);
                    }
                    _ => {
                        parsed_attributes.raw_attributes.push(attribute.clone());
                    }
                };
            } else {
                parsed_attributes.raw_attributes.push(attribute.clone());
            }
        }

        Ok(parsed_attributes)
    }
}
