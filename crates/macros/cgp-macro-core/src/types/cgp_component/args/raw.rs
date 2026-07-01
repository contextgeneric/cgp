use syn::parse::{End, Parse, ParseStream};
use syn::token::{Colon, Comma};
use syn::{Error, Ident};

use crate::types::ident::IdentWithTypeGenerics;

/// The attribute args exactly as written, before defaults are applied. Parses
/// either a bare provider identifier or the `key: value` form; [`CgpComponentArgs`]
/// resolves the defaults via `TryFrom`.
#[derive(Default)]
pub struct CgpComponentRawArgs {
    pub context_ident: Option<Ident>,
    pub provider_ident: Option<Ident>,
    pub component_name: Option<IdentWithTypeGenerics>,
}

impl Parse for CgpComponentRawArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(End) {
            let provider_ident = input.parse()?;

            return Ok(CgpComponentRawArgs {
                provider_ident: Some(provider_ident),
                ..Default::default()
            });
        }

        let mut args = Self::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;

            let _: Colon = input.parse()?;

            match key.to_string().as_str() {
                "name" => {
                    if args.component_name.is_some() {
                        return Err(Error::new(key.span(), "duplicate key is not allowed"));
                    }

                    args.component_name = Some(input.parse()?);
                }
                "context" => {
                    if args.context_ident.is_some() {
                        return Err(Error::new(key.span(), "duplicate key is not allowed"));
                    }

                    args.context_ident = Some(input.parse()?);
                }
                "provider" => {
                    if args.provider_ident.is_some() {
                        return Err(Error::new(key.span(), "duplicate key is not allowed"));
                    }

                    args.provider_ident = Some(input.parse()?);
                }
                _ => {
                    return Err(Error::new(key.span(), format!("unknown key {key}")));
                }
            }

            if input.parse::<Option<Comma>>()?.is_none() {
                break;
            }
        }

        Ok(args)
    }
}
