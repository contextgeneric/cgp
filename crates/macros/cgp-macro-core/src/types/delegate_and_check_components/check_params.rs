use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Type};

pub enum CheckParamsAttribute {
    None,
    Skip,
    Multi(Punctuated<Type, Comma>),
}

impl CheckParamsAttribute {
    pub fn parse_attributes(attributes: &[Attribute]) -> syn::Result<Self> {
        if attributes.is_empty() {
            return Ok(Self::None);
        }

        let attribute = &attributes[0];

        if attribute.path().is_ident("check_params") {
            let params = attribute.parse_args_with(Punctuated::parse_terminated)?;
            Ok(CheckParamsAttribute::Multi(params))
        } else if attribute.path().is_ident("skip_check") {
            // TODO: validate that the attribute args are empty

            Ok(CheckParamsAttribute::Skip)
        } else {
            Err(syn::Error::new(
                attribute.span(),
                "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            ))
        }
    }
}
