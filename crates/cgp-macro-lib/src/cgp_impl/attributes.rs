use core::mem;

use syn::Attribute;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::cgp_fn::UseTypeSpec;
use crate::parse::SimpleType;

pub fn parse_impl_attributes(attributes: &mut Vec<Attribute>) -> syn::Result<ImplAttributes> {
    let mut parsed_attributes = ImplAttributes::default();

    let in_attributes = mem::take(attributes);

    for attribute in in_attributes.into_iter() {
        if let Some(ident) = attribute.path().get_ident() {
            if ident == "uses" {
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

#[derive(Default)]
pub struct ImplAttributes {
    pub uses: Vec<SimpleType>,
    pub use_type: Vec<UseTypeSpec>,
}
