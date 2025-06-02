use alloc::format;
use std::collections::BTreeMap;

use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{End, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt, Paren};
use syn::{parenthesized, parse2, Error, Ident, Type};

use crate::parse::Entries;

pub struct ComponentSpec {
    pub provider_name: Ident,
    pub context_type: Ident,
    pub component_name: Ident,
    pub component_params: Punctuated<Ident, Comma>,
    pub use_delegate_spec: Vec<UseDelegateSpec>,
}

pub struct ComponentNameSpec {
    pub component_name: Ident,
    pub component_params: Punctuated<Ident, Comma>,
}

static VALID_KEYS: [&str; 4] = ["context", "provider", "name", "derive_delegate"];

impl Parse for ComponentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(End) {
            let provider_name: Ident = input.parse()?;

            let context_type = Ident::new("Context", Span::call_site());

            let component_name =
                Ident::new(&format!("{provider_name}Component"), provider_name.span());

            let component_params = Punctuated::new();

            Ok(Self {
                provider_name,
                context_type,
                component_name,
                component_params,
                use_delegate_spec: Vec::new(),
            })
        } else {
            let Entries { entries } = input.parse()?;
            Self::from_entries(&entries)
        }
    }
}

impl ComponentSpec {
    pub fn validate_entries(entries: &BTreeMap<String, Type>) -> syn::Result<()> {
        for key in entries.keys() {
            if !VALID_KEYS.iter().any(|valid| valid == key) {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        r#"invalid key in component spec: {key}. the following keys are valid: "context", "provider", "name"."#
                    ),
                ));
            }
        }

        Ok(())
    }

    pub fn from_entries(entries: &BTreeMap<String, Type>) -> syn::Result<Self> {
        Self::validate_entries(entries)?;

        let context_type: Ident = {
            let raw_context_type = entries.get("context");

            if let Some(context_type) = raw_context_type {
                syn::parse2(context_type.to_token_stream())?
            } else {
                Ident::new("Context", Span::call_site())
            }
        };

        let provider_name: Ident = {
            let raw_provider_name = entries
                .get("provider")
                .ok_or_else(|| Error::new(Span::call_site(), "expect provider name to be given"))?;

            syn::parse2(raw_provider_name.to_token_stream())?
        };

        let (component_name, component_params) = {
            let raw_component_name = entries.get("name");

            if let Some(raw_component_name) = raw_component_name {
                let ComponentNameSpec {
                    component_name,
                    component_params,
                } = syn::parse2(raw_component_name.to_token_stream())?;
                (component_name, component_params)
            } else {
                (
                    Ident::new(&format!("{}Component", provider_name), provider_name.span()),
                    Punctuated::default(),
                )
            }
        };

        let use_delegate_spec = match entries.get("derive_delegate") {
            Some(entry) => {
                let spec = parse2(entry.to_token_stream())?;
                vec![spec]
            }
            None => Vec::new(),
        };

        Ok(ComponentSpec {
            component_name,
            provider_name,
            context_type,
            component_params,
            use_delegate_spec,
        })
    }
}

impl Parse for ComponentNameSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name: Ident = input.parse()?;

        let component_params = if input.peek(Lt) {
            let _: Lt = input.parse()?;

            let component_params: Punctuated<Ident, Comma> =
                Punctuated::parse_separated_nonempty(input)?;

            let _: Gt = input.parse()?;

            component_params
        } else {
            Punctuated::default()
        };

        Ok(Self {
            component_name,
            component_params,
        })
    }
}

pub struct UseDelegateSpec {
    pub wrapper: Ident,
    pub params: Punctuated<Ident, Comma>,
}

impl Parse for UseDelegateSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let idents = if input.peek(Paren) {
            let body;
            parenthesized!(body in input);
            let idents = Punctuated::parse_terminated(&body)?;
            if idents.is_empty() {
                return Err(Error::new(
                    body.span(),
                    "expect non-empty tuple list of identifiers in use_delegate_spec",
                ));
            }

            idents
        } else {
            let ident: Ident = input.parse()?;
            Punctuated::from_iter([ident])
        };

        let _: Gt = input.parse()?;
        Ok(Self {
            wrapper,
            params: idents,
        })
    }
}
