use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{As, Brace, Colon, Comma, Gt, Lt};
use syn::{Ident, Type, TypeParamBound, braced, parse2};

use crate::parse::SimpleType;

pub struct UseTypeSpec {
    pub trait_path: SimpleType,
    pub type_idents: Vec<UseTypeIdent>,
}

pub struct UseTypeIdent {
    pub type_ident: Ident,
    pub as_alias: Option<Ident>,
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

    pub fn trait_bounds(
        context_type: &TokenStream,
        specs: &[Self],
    ) -> syn::Result<Vec<TypeParamBound>> {
        let mut bounds = Vec::new();

        let mut past_specs = Vec::new();

        for use_type in specs.iter() {
            let type_equalities = use_type.find_type_equalities(context_type, &past_specs)?;

            if type_equalities.is_empty() {
                bounds.push(parse2(use_type.trait_path.to_token_stream())?);
                past_specs.push(use_type);
            } else {
                let mut constraints: Punctuated<TokenStream, Comma> = Punctuated::new();

                for (alias_ident, equal_target) in type_equalities.into_iter() {
                    constraints.push(quote! {
                        #alias_ident = #equal_target
                    });
                }

                let trait_path = &use_type.trait_path;
                let bound = quote! {
                    #trait_path < #constraints >
                };

                bounds.push(parse2(bound)?);
            }
        }

        Ok(bounds)
    }

    pub fn find_type_equalities(
        &self,
        context_type: &TokenStream,
        past_specs: &[&UseTypeSpec],
    ) -> syn::Result<Vec<(Ident, Type)>> {
        let mut equalities = Vec::new();

        for type_ident in self.type_idents.iter() {
            if let Some(equality) = Self::find_type_equality(context_type, type_ident, past_specs)?
            {
                equalities.push(equality);
            }
        }

        Ok(equalities)
    }

    pub fn find_type_equality(
        context_type: &TokenStream,
        current_ident: &UseTypeIdent,
        past_specs: &[&UseTypeSpec],
    ) -> syn::Result<Option<(Ident, Type)>> {
        for past_spec in past_specs.iter() {
            for type_ident in past_spec.type_idents.iter() {
                let current_alias = current_ident.alias_ident();

                if current_alias == type_ident.alias_ident() {
                    if current_ident.as_alias.is_none() || type_ident.as_alias.is_none() {
                        return Err(syn::Error::new_spanned(
                            &current_ident.type_ident,
                            "Multiple abstract types with the same name must be aliased explicitly with `as`",
                        ));
                    }

                    let trait_path = &past_spec.trait_path;
                    let type_ident = &current_ident.type_ident;

                    let equal_target: Type = parse2(quote! {
                        <#context_type as #trait_path>::#type_ident
                    })?;

                    return Ok(Some((current_alias.clone(), equal_target)));
                }
            }
        }

        Ok(None)
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

        Ok(Self {
            type_ident,
            as_alias,
        })
    }
}
