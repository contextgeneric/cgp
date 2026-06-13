use core::mem;

use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, TypeParamBound};

use crate::types::attributes::{PrefixAttribute, UseTypeAttribute, UseTypeAttributes};

#[derive(Default, Clone)]
pub struct CgpComponentAttributes {
    pub extend: Vec<TypeParamBound>,
    pub use_type: UseTypeAttributes,
    pub prefixes: Vec<PrefixAttribute>,
}

impl CgpComponentAttributes {
    pub fn parse(attributes: &mut Vec<Attribute>) -> syn::Result<Self> {
        let mut parsed_attributes = CgpComponentAttributes::default();

        let in_attributes = mem::take(attributes);

        for attribute in in_attributes.into_iter() {
            if let Some(ident) = attribute.path().get_ident() {
                if ident == "extend" {
                    let extend_bound = attribute
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;
                    parsed_attributes.extend.extend(extend_bound);
                } else if ident == "use_type" {
                    let use_type_specs = attribute
                        .parse_args_with(Punctuated::<UseTypeAttribute, Comma>::parse_terminated)?;

                    for use_type_spec in use_type_specs.iter() {
                        for type_ident in use_type_spec.type_idents.iter() {
                            if let Some(equals) = &type_ident.equals {
                                return Err(syn::Error::new_spanned(
                                    equals,
                                    "Type equality constraints cannot be used in component trait definition",
                                ));
                            }
                        }
                    }

                    parsed_attributes.use_type.attributes.extend(use_type_specs);
                } else if ident == "prefix" {
                    let namespace_specs = attribute.parse_args_with(PrefixAttribute::parse)?;
                    parsed_attributes.prefixes.push(namespace_specs);
                } else {
                    attributes.push(attribute);
                }
            } else {
                attributes.push(attribute);
            }
        }

        Ok(parsed_attributes)
    }
}
