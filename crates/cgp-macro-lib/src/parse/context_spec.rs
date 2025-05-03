use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, Lt};
use syn::{AngleBracketedGenericArguments, Ident};

use crate::parse::TypeGenerics;

pub struct ContextSpec {
    pub provider_name: Ident,
    pub provider_generics: Option<TypeGenerics>,
    pub preset: Option<(Ident, Option<AngleBracketedGenericArguments>)>,
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
                let path = input.parse()?;

                let generics = if input.peek(Lt) {
                    Some(input.parse()?)
                } else {
                    None
                };

                Some((path, generics))
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
