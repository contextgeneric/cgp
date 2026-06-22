use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Pound};
use syn::{Attribute, Type};

pub enum CheckParamsAttribute {
    None,
    Skip,
    Multi(Punctuated<Type, Comma>),
}

impl Parse for CheckParamsAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(Pound) {
            return Ok(Self::None);
        }

        let attributes = input.call(Attribute::parse_outer)?;

        let [attribute]: [Attribute; 1] = attributes
            .try_into()
            .map_err(|_| input.error("Expected exactly one key attribute"))?;

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
