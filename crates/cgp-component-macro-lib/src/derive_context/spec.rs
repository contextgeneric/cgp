use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, Lt};
use syn::{AngleBracketedGenericArguments, Ident};

pub struct ContextSpec {
    pub provider_name: Ident,
    pub preset: Option<(Ident, Option<AngleBracketedGenericArguments>)>,
}

impl Parse for ContextSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let provider_name = input.parse()?;

        let colon: Option<Colon> = input.parse()?;

        let preset = match colon {
            Some(_) => {
                let preset_name: Ident = input.parse()?;
                let preset_generics = if input.peek(Lt) {
                    Some(input.parse()?)
                } else {
                    None
                };

                Some((preset_name, preset_generics))
            }
            None => None,
        };

        Ok(Self {
            provider_name,
            preset,
        })
    }
}
