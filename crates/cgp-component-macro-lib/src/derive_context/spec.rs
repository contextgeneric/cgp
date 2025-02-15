use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::Ident;

use crate::parse::SimpleType;

pub struct ContextSpec {
    pub provider_name: Ident,
    pub preset: Option<SimpleType>,
}

impl Parse for ContextSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let provider_name = input.parse()?;

        let colon: Option<Colon> = input.parse()?;

        let preset = match colon {
            Some(_) => Some(input.parse()?),
            None => None,
        };

        Ok(Self {
            provider_name,
            preset,
        })
    }
}
