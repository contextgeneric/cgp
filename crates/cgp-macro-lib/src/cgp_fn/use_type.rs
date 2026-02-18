use syn::parse::{Parse, ParseStream};
use syn::token::{As, Brace, Colon, Comma, Eq, Gt, Lt};
use syn::{Ident, Type, braced};

use crate::parse::SimpleType;

pub struct UseTypeSpec {
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
        let trait_path = if input.peek(Lt) {
            let _: Lt = input.parse()?;
            let trait_path: SimpleType = input.parse()?;
            let _: Gt = input.parse()?;
            trait_path
        } else {
            let name: Ident = input.parse()?;
            SimpleType {
                name,
                generics: None,
            }
        };

        let _: Colon = input.parse()?;
        let _: Colon = input.parse()?;

        let type_idents: Vec<UseTypeIdent> = if input.peek(Brace) {
            let content;
            braced!(content in input);
            content
                .parse_terminated(UseTypeIdent::parse, Comma)?
                .into_iter()
                .collect()
        } else {
            let ident: Ident = input.parse()?;
            vec![UseTypeIdent {
                type_ident: ident,
                as_alias: None,
                equals: None,
            }]
        };

        Ok(Self {
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
