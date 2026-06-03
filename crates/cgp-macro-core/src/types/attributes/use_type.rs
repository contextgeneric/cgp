use syn::parse::{Parse, ParseStream};
use syn::token::{As, At, Brace, Colon, Comma, Eq, Gt, Lt};
use syn::visit_mut::VisitMut;
use syn::{Ident, ItemImpl, ItemTrait, Type, braced, parse_quote};

use crate::types::ident::IdentWithTypeArgs;
use crate::visitors::SubstituteAbstractType;

#[derive(Default)]
pub struct UseTypeAttributes {
    pub attributes: Vec<UseTypeAttribute>,
}

pub struct UseTypeAttribute {
    pub context_type: Type,
    pub trait_path: IdentWithTypeArgs,
    pub type_idents: Vec<UseTypeIdent>,
}

pub struct UseTypeIdent {
    pub type_ident: Ident,
    pub as_alias: Option<Ident>,
    pub equals: Option<Type>,
}

impl UseTypeAttributes {
    pub fn substitute_abstract_types_in_item_trait(&self, item_trait: &mut ItemTrait) {
        for type_spec in self.attributes.iter().rev() {
            SubstituteAbstractType { type_spec }.visit_item_trait_mut(item_trait);
        }
    }

    pub fn substitute_abstract_types_in_item_impl(&self, item_impl: &mut ItemImpl) {
        for type_spec in self.attributes.iter().rev() {
            SubstituteAbstractType { type_spec }.visit_item_impl_mut(item_impl);
        }
    }
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

impl UseTypeIdent {
    pub fn alias_ident(&self) -> &Ident {
        self.as_alias.as_ref().unwrap_or(&self.type_ident)
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
            (parse_quote! { Self }, input)
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
