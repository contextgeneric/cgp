use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, Lt};

use crate::parse::{SimpleType, TypeGenerics};

pub struct ContextSpec {
    pub provider_name: Ident,
    pub provider_generics: Option<TypeGenerics>,
    pub preset: Option<SimpleType>,
}

impl Parse for ContextSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let provider_name = input.parse()?;

        let provider_generics = if input.peek(Lt) {
            Some(input.parse()?)
        } else {
            None
        };

        let colon: Option<Colon> = input.parse()?;

        let preset = match colon {
            Some(_) => {
                let preset = input.parse()?;
                Some(preset)
            }
            None => None,
        };

        Ok(Self {
            provider_name,
            provider_generics,
            preset,
        })
    }
}
