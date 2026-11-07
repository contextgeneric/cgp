use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, TypeParamBound};

use crate::types::attributes::{
    DefaultImplAttribute, DefaultImplAttributes, UseProviderAttribute, UseProviderAttributes,
    UseTypeAttribute, UseTypeAttributes, UsesAttributes,
};

#[derive(Default)]
/// The modifier attributes a `#[cgp_impl]` block carries.
pub struct CgpImplAttributes {
    /// The `#[uses]` imports.
    pub uses: UsesAttributes,
    /// The `#[use_type]` imports.
    pub use_type: UseTypeAttributes,
    /// The `#[use_provider]` imports.
    pub use_provider: UseProviderAttributes,
    /// The `#[default_impl]` registrations.
    pub default_impls: DefaultImplAttributes,
    /// The attributes the macro passes through.
    pub raw_attributes: Vec<Attribute>,
}

impl CgpImplAttributes {
    /// Parses the block's modifier attributes.
    pub fn parse(attributes: &Vec<Attribute>) -> syn::Result<CgpImplAttributes> {
        let mut parsed_attributes = CgpImplAttributes::default();

        for attribute in attributes {
            if let Some(ident) = attribute.path().get_ident() {
                match ident.to_string().as_ref() {
                    "uses" => {
                        let uses = attribute.parse_args_with(
                            Punctuated::<TypeParamBound, Comma>::parse_terminated,
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
