use core::iter;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Comma, Lt, Pound};
use syn::{Attribute, Ident, Type, braced, bracketed, parse2};

use crate::parse::{DelegateMode, DelegateValue, ImplGenerics, SimpleType};

pub struct DelegateAndCheckSpec {
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub entries: Punctuated<DelegateAndCheckEntry, Comma>,
}

#[derive(Clone)]
pub struct DelegateAndCheckEntry {
    pub keys: Punctuated<DelegateAndCheckKey, Comma>,
    pub mode: DelegateMode,
    pub value: DelegateValue,
}

#[derive(Clone)]
pub struct DelegateAndCheckKey {
    pub component_type: Type,
    pub check_params: Option<Punctuated<Type, Comma>>,
}

impl Parse for DelegateAndCheckSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let m_trait_name = parse_check_trait_name(input)?;

        let context_type: Type = input.parse()?;

        let trait_name = match m_trait_name {
            Some(ident) => ident,
            None => {
                let context_type: SimpleType = parse2(context_type.to_token_stream())?;
                Ident::new(
                    &format!("__CanUse{}", context_type.name),
                    context_type.span(),
                )
            }
        };

        let entries = {
            let body;
            braced!(body in input);
            Punctuated::parse_terminated(&body)?
        };

        Ok(Self {
            impl_generics,
            trait_name,
            context_type,
            entries,
        })
    }
}

impl Parse for DelegateAndCheckEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_params = parse_check_params(input)?;

        let mut keys = if input.peek(Bracket) {
            let body;
            bracketed!(body in input);
            Punctuated::parse_terminated(&body)?
        } else {
            let key: DelegateAndCheckKey = input.parse()?;
            Punctuated::from_iter(iter::once(key))
        };

        if let Some(check_params) = check_params {
            for key in &mut keys {
                key.check_params
                    .get_or_insert_default()
                    .extend(check_params.clone());
            }
        }

        let mode = input.parse()?;

        let value = input.parse()?;

        Ok(Self { keys, mode, value })
    }
}

impl Parse for DelegateAndCheckKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_params = parse_check_params(input)?;

        let component_type: Type = input.parse()?;
        Ok(Self {
            component_type,
            check_params,
        })
    }
}

pub fn parse_check_trait_name(input: ParseStream) -> syn::Result<Option<Ident>> {
    if input.peek(Pound) {
        let attributes = input.call(Attribute::parse_outer)?;

        let [attribute]: [Attribute; 1] = attributes
            .try_into()
            .map_err(|_| input.error("Expected exactly one attribute for the check trait name"))?;

        if !attribute.path().is_ident("check_trait") {
            return Err(syn::Error::new(
                attribute.span(),
                "Expected `#[check_trait]` attribute for specifying the check trait name",
            ));
        }

        let ident: Ident = attribute.parse_args()?;
        Ok(Some(ident))
    } else {
        Ok(None)
    }
}

pub fn parse_check_params(input: ParseStream) -> syn::Result<Option<Punctuated<Type, Comma>>> {
    if input.peek(Pound) {
        let attributes = input.call(Attribute::parse_outer)?;

        let [attribute]: [Attribute; 1] = attributes
            .try_into()
            .map_err(|_| input.error("Expected exactly one key attribute"))?;

        let check_params = if attribute.path().is_ident("check_params") {
            attribute.parse_args_with(Punctuated::parse_terminated)?
        } else if attribute.path().is_ident("skip_check") {
            Punctuated::new()
        } else {
            return Err(syn::Error::new(
                attribute.span(),
                "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            ));
        };

        Ok(Some(check_params))
    } else {
        Ok(None)
    }
}
