use core::mem;

use cgp_macro_core::types::ident::IdentWithTypeArgs;
use syn::Attribute;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::cgp_fn::UseTypeSpec;
use crate::cgp_impl::use_provider::UseProviderSpec;

pub fn parse_impl_attributes(attributes: &mut Vec<Attribute>) -> syn::Result<ImplAttributes> {
    let mut parsed_attributes = ImplAttributes::default();

    let in_attributes = mem::take(attributes);

    for attribute in in_attributes.into_iter() {
        if let Some(ident) = attribute.path().get_ident() {
            if ident == "uses" {
                let uses = attribute
                    .parse_args_with(Punctuated::<IdentWithTypeArgs, Comma>::parse_terminated)?;
                parsed_attributes.uses.extend(uses);
            } else if ident == "use_type" {
                let use_type = attribute
                    .parse_args_with(Punctuated::<UseTypeSpec, Comma>::parse_terminated)?;
                parsed_attributes.use_type.extend(use_type);
            } else if ident == "use_provider" {
                let use_provider = attribute
                    .parse_args_with(Punctuated::<UseProviderSpec, Comma>::parse_terminated)?;
                parsed_attributes.use_provider.extend(use_provider);
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
    pub uses: Vec<IdentWithTypeArgs>,
    pub use_type: Vec<UseTypeSpec>,
    pub use_provider: Vec<UseProviderSpec>,
}
