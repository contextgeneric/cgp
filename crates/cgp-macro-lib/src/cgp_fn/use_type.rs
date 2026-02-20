use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::{As, At, Brace, Colon, Comma, Eq, Gt, Lt};
use syn::{Ident, Type, braced, parse_quote, parse2};

use crate::parse::SimpleType;

pub struct UseTypeSpec {
    pub context_type: Type,
    pub trait_path: SimpleType,
    pub type_idents: Vec<UseTypeIdent>,
}

pub struct UseTypeIdent {
    pub type_ident: Ident,
    pub as_alias: Option<Ident>,
    pub equals: Option<Type>,
}

impl UseTypeSpec {
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

impl UseTypeIdent {
    pub fn alias_ident(&self) -> &Ident {
        self.as_alias.as_ref().unwrap_or(&self.type_ident)
    }
}

impl Parse for UseTypeSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;

        let (context_type, body) = if input.peek(At) {
            let _: At = input.parse()?;

            let context_type: SimpleType = input.parse()?;
            let context_type = parse2(context_type.into_token_stream())?;

            let _: Colon = input.parse()?;
            let _: Colon = input.parse()?;

            if input.peek(Brace) {
                braced!(body in input);
                (context_type, &body)
            } else {
                (context_type, input)
            }
        } else {
            (parse_quote! { Self }, input)
        };

        let trait_path = if body.peek(Lt) {
            let _: Lt = body.parse()?;
            let trait_path: SimpleType = body.parse()?;
            let _: Gt = body.parse()?;
            trait_path
        } else {
            let name: Ident = body.parse()?;
            SimpleType {
                name,
                generics: None,
            }
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

impl Parse for UseTypeIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_ident: Ident = input.parse()?;

        let as_alias = if input.peek(As) {
            let _: As = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        let equals = if input.peek(Eq) {
            let _: Eq = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            type_ident,
            as_alias,
            equals,
        })
    }
}
