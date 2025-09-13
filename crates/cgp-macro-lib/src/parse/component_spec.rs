use alloc::format;
use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use syn::parse::{End, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma, Gt, Lt, Paren};
use syn::{bracketed, parenthesized, parse2, Error, Ident};

use crate::parse::Entries;

pub struct ComponentSpec {
    pub provider_name: Ident,
    pub context_type: Ident,
    pub component_name: Ident,
    pub component_params: Punctuated<Ident, Comma>,
    pub use_delegate_spec: Vec<DeriveDelegateSpec>,
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

            let context_type = Ident::new("__Context__", Span::call_site());

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
    pub fn validate_entries(entries: &BTreeMap<String, TokenStream>) -> syn::Result<()> {
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

    pub fn from_entries(entries: &BTreeMap<String, TokenStream>) -> syn::Result<Self> {
        Self::validate_entries(entries)?;

        let context_type: Ident = {
            let raw_context_type = entries.get("context");

            if let Some(context_type) = raw_context_type {
                syn::parse2(context_type.clone())?
            } else {
                Ident::new("__Context__", Span::call_site())
            }
        };

        let provider_name: Ident = {
            let raw_provider_name = entries
                .get("provider")
                .ok_or_else(|| Error::new(Span::call_site(), "expect provider name to be given"))?;

            syn::parse2(raw_provider_name.clone())?
        };

        let (component_name, component_params) = {
            let raw_component_name = entries.get("name");

            if let Some(raw_component_name) = raw_component_name {
                let ComponentNameSpec {
                    component_name,
                    component_params,
                } = syn::parse2(raw_component_name.clone())?;
                (component_name, component_params)
            } else {
                (
                    Ident::new(&format!("{provider_name}Component"), provider_name.span()),
                    Punctuated::default(),
                )
            }
        };

        let use_delegate_spec = match entries.get("derive_delegate") {
            Some(entry) => {
                let DeriveDelegateSpecs { specs } = parse2(entry.clone())?;
                specs
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

pub struct DeriveDelegateSpec {
    pub wrapper: Ident,
    pub params: Punctuated<Ident, Comma>,
}

impl Parse for DeriveDelegateSpec {
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

pub struct DeriveDelegateSpecs {
    pub specs: Vec<DeriveDelegateSpec>,
}

impl Parse for DeriveDelegateSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let body;
            bracketed!(body in input);

            let specs = <Punctuated<DeriveDelegateSpec, Comma>>::parse_terminated(&body)?;
            Ok(Self {
                specs: Vec::from_iter(specs),
            })
        } else {
            let spec = input.parse()?;
            Ok(Self { specs: vec![spec] })
        }
    }
}
