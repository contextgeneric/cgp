use core::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, TypeParamBound};

use crate::cgp_fn::UseTypeSpec;

pub fn parse_component_attributes(
    attributes: &mut Vec<Attribute>,
) -> syn::Result<ComponentAttributes> {
    let mut parsed_attributes = ComponentAttributes::default();

    let in_attributes = mem::take(attributes);

    for attribute in in_attributes.into_iter() {
        if let Some(ident) = attribute.path().get_ident() {
            if ident == "extend" {
                let extend_bound = attribute
                    .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;
                parsed_attributes.extend.extend(extend_bound);
            } else if ident == "use_type" {
                let use_type_specs = attribute
                    .parse_args_with(Punctuated::<UseTypeSpec, Comma>::parse_terminated)?;

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

                parsed_attributes.use_type.extend(use_type_specs);
            } else {
                attributes.push(attribute);
            }
        } else {
            attributes.push(attribute);
        }
    }

    Ok(parsed_attributes)
}

#[derive(Default)]
pub struct ComponentAttributes {
    pub extend: Vec<TypeParamBound>,
    pub use_type: Vec<UseTypeSpec>,
}
