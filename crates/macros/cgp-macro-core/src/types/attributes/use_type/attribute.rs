use syn::parse::{Parse, ParseStream};
use syn::token::{At, Brace, Comma, Dot};
use syn::{Ident, Type};

use crate::parse_internal;
use crate::types::attributes::UseTypeIdent;
use crate::types::ident::PathWithTypeArgs;

/// One `#[use_type(...)]` import spec: a rewrite target (`Self` or an `@Context`),
/// the owning trait path, and one or more associated types to import from it.
#[derive(Clone)]
pub struct UseTypeAttribute {
    pub context_type: Type,
    pub trait_path: PathWithTypeArgs,
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
        // A `.` (not `::`) separates the context, trait, and associated type. This
        // keeps the trait unambiguous even when it is a full path such as
        // `foo::bar::HasScalarType`: `::` stays inside the path, and the trailing
        // `.` marks where the associated type begins.
        let context_type: Type = if input.peek(At) {
            let _: At = input.parse()?;
            let context: PathWithTypeArgs = input.parse()?;
            let _: Dot = input.parse()?;
            context.into()
        } else {
            parse_internal! { Self }
        };

        let trait_path: PathWithTypeArgs = input.parse()?;

        let _: Dot = input.parse()?;

        let type_idents: Vec<UseTypeIdent> = if input.peek(Brace) {
            let content;
            syn::braced!(content in input);
            content
                .parse_terminated(UseTypeIdent::parse, Comma)?
                .into_iter()
                .collect()
        } else {
            vec![input.parse()?]
        };

        Ok(Self {
            context_type,
            trait_path,
            type_idents,
        })
    }
}
