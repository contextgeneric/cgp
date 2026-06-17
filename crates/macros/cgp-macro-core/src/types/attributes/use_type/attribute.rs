use syn::parse::{Parse, ParseStream};
use syn::token::{At, Brace, Colon, Comma, Gt, Lt};
use syn::{Ident, Type, braced};

use crate::parse_internal;
use crate::types::attributes::UseTypeIdent;
use crate::types::ident::IdentWithTypeArgs;

#[derive(Clone)]
pub struct UseTypeAttribute {
    pub context_type: Type,
    pub trait_path: IdentWithTypeArgs,
    pub type_idents: Vec<UseTypeIdent>,
}

impl UseTypeAttribute {
    pub fn replace_ident(&self, ident: &Ident) -> Option<Ident> {
        for type_ident in &self.type_idents {
            if type_ident.alias_ident() == ident {
                let mut new_ident = type_ident.type_ident.clone();
                new_ident.set_span(ident.span());
                return Some(new_ident);
            }
        }

        None
    }
}

impl Parse for UseTypeAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;

        let (context_type, body) = if input.peek(At) {
            let _: At = input.parse()?;

            let context_type: Type = input.parse::<IdentWithTypeArgs>()?.into();

            let _: Colon = input.parse()?;
            let _: Colon = input.parse()?;

            if input.peek(Brace) {
                braced!(body in input);
                (context_type, &body)
            } else {
                (context_type, input)
            }
        } else {
            (parse_internal! { Self }, input)
        };

        let trait_path = if body.peek(Lt) {
            let _: Lt = body.parse()?;
            let trait_path: IdentWithTypeArgs = body.parse()?;
            let _: Gt = body.parse()?;
            trait_path
        } else {
            let name: Ident = body.parse()?;
            name.into()
        };

        let _: Colon = body.parse()?;
        let _: Colon = body.parse()?;

        let type_idents: Vec<UseTypeIdent> = if body.peek(Brace) {
            let content;
            braced!(content in body);
            content
                .parse_terminated(UseTypeIdent::parse, Comma)?
                .into_iter()
                .collect()
        } else {
            let ident: Ident = body.parse()?;
            vec![UseTypeIdent {
                type_ident: ident,
                as_alias: None,
                equals: None,
            }]
        };

        Ok(Self {
            context_type,
            trait_path,
            type_idents,
        })
    }
}
