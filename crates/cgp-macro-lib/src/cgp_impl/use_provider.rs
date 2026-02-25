use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::{Type, parse_quote};

use crate::parse::SimpleType;

pub struct UseProviderSpec {
    pub context_type: Type,
    pub provider_type: Type,
    pub provider_trait_bounds: Punctuated<SimpleType, Plus>,
}

impl Parse for UseProviderSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context_type = parse_quote!(Self);
        let provider_type = input.parse()?;

        let _: Colon = input.parse()?;
        let provider_trait_bounds = Punctuated::parse_terminated(input)?;

        Ok(Self {
            context_type,
            provider_type,
            provider_trait_bounds,
        })
    }
}
