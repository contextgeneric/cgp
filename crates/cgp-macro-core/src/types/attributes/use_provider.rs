use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::{Type, parse_quote};

use crate::types::ident::IdentWithTypeArgs;

pub struct UseProviderAttribute {
    pub context_type: Type,
    pub provider_type: Type,
    pub colon: Colon,
    pub provider_trait_bounds: Punctuated<IdentWithTypeArgs, Plus>,
}

impl Parse for UseProviderAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context_type = parse_quote!(Self);
        let provider_type = input.parse()?;

        let colon: Colon = input.parse()?;
        let provider_trait_bounds = Punctuated::parse_terminated(input)?;

        Ok(Self {
            context_type,
            provider_type,
            colon,
            provider_trait_bounds,
        })
    }
}
