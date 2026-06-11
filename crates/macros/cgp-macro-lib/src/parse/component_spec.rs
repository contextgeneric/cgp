use alloc::format;
use std::collections::BTreeMap;

use cgp_macro_core::types::cgp_component::DeriveDelegateAttributes;
use cgp_macro_core::types::ident::IdentWithTypeGenerics;
use proc_macro2::{Span, TokenStream};
use syn::parse::{End, Parse, ParseStream};
use syn::{Error, Ident, parse2};

use crate::parse::Entries;

pub struct CgpComponentArgs {
    pub provider_ident: Ident,
    pub context_ident: Ident,
    pub component_name: IdentWithTypeGenerics,
    pub derive_delegate_attributes: DeriveDelegateAttributes,
}

static VALID_KEYS: [&str; 4] = ["context", "provider", "name", "derive_delegate"];

impl Parse for CgpComponentArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(End) {
            let provider_name: Ident = input.parse()?;

            let context_type = Ident::new("__Context__", Span::call_site());

            let component_name =
                Ident::new(&format!("{provider_name}Component"), provider_name.span());

            Ok(Self {
                provider_ident: provider_name,
                context_ident: context_type,
                component_name: component_name.into(),
                derive_delegate_attributes: Default::default(),
            })
        } else {
            let Entries { entries } = input.parse()?;
            Self::from_entries(&entries)
        }
    }
}

impl CgpComponentArgs {
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

        let component_name = {
            let raw_component_name = entries.get("name");

            if let Some(raw_component_name) = raw_component_name {
                parse2(raw_component_name.clone())?
            } else {
                IdentWithTypeGenerics::from(Ident::new(
                    &format!("{provider_name}Component"),
                    provider_name.span(),
                ))
            }
        };

        let derive_delegate_attributes = match entries.get("derive_delegate") {
            Some(entry) => parse2(entry.clone())?,
            None => Default::default(),
        };

        Ok(CgpComponentArgs {
            component_name,
            provider_ident: provider_name,
            context_ident: context_type,
            derive_delegate_attributes,
        })
    }
}
