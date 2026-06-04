use syn::Attribute;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::types::attributes::{
    UseProviderAttribute, UseProviderAttributes, UseTypeAttribute, UseTypeAttributes,
    UsesAttributes,
};
use crate::types::ident::IdentWithTypeArgs;

#[derive(Default)]
pub struct ImplAttributes {
    pub uses: UsesAttributes,
    pub use_type: UseTypeAttributes,
    pub use_provider: UseProviderAttributes,
    pub raw_attributes: Vec<Attribute>,
}

impl ImplAttributes {
    pub fn parse(attributes: &Vec<Attribute>) -> syn::Result<ImplAttributes> {
        let mut parsed_attributes = ImplAttributes::default();

        for attribute in attributes {
            if let Some(ident) = attribute.path().get_ident() {
                match ident.to_string().as_ref() {
                    "uses" => {
                        let uses = attribute.parse_args_with(
                            Punctuated::<IdentWithTypeArgs, Comma>::parse_terminated,
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
