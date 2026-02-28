use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Colon, Comma, Lt, Pound, Where};
use syn::{Attribute, Ident, Type, WhereClause, braced, bracketed, parse2};

use crate::parse::{ImplGenerics, SimpleType};

pub struct CheckComponentsSpecs {
    pub specs: Vec<CheckComponents>,
}

pub struct CheckComponents {
    pub check_providers: Option<Punctuated<Type, Comma>>,
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub where_clause: WhereClause,
    pub check_entries: CheckEntries,
}

pub struct CheckEntries {
    pub entries: Vec<CheckEntry>,
}

pub struct CheckEntry {
    pub component_type: Type,
    pub component_params: Option<Type>,
    pub span: Span,
}

struct ParseCheckEntries {
    pub entries: Vec<CheckEntry>,
}

impl Parse for CheckComponentsSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut specs = Vec::new();

        while !input.is_empty() {
            let spec: CheckComponents = input.parse()?;
            specs.push(spec);
        }

        Ok(Self { specs })
    }
}

impl Parse for CheckComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut check_providers: Option<Punctuated<Type, Comma>> = None;
        let mut m_check_trait_name: Option<Ident> = None;

        if input.peek(Pound) {
            let attributes = input.call(Attribute::parse_outer)?;

            for attribute in attributes {
                if attribute.path().is_ident("check_providers") {
                    let provider_types: Punctuated<Type, Comma> =
                        attribute.parse_args_with(Punctuated::parse_terminated)?;

                    check_providers
                        .get_or_insert_default()
                        .extend(provider_types);
                } else if attribute.path().is_ident("check_trait") {
                    let check_trait_name: Ident = attribute.parse_args()?;

                    if m_check_trait_name.is_some() {
                        return Err(syn::Error::new(
                            attribute.span(),
                            "Multiple `#[check_trait]` attributes found. Expected at most one.",
                        ));
                    }

                    m_check_trait_name = Some(check_trait_name);
                } else {
                    return Err(syn::Error::new(
                        attribute.span(),
                        format!("Invalid attribute {}", attribute.to_token_stream()),
                    ));
                }
            }
        };

        let impl_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let context_type: Type = input.parse()?;

        let trait_name = if let Some(check_trait_name) = m_check_trait_name {
            check_trait_name
        } else {
            let context_type: SimpleType = parse2(context_type.to_token_stream())?;

            Ident::new(
                &format!("__Check{}", context_type.name),
                context_type.span(),
            )
        };

        let where_clause = if input.peek(Where) {
            input.parse()?
        } else {
            WhereClause {
                where_token: Where(Span::call_site()),
                predicates: Punctuated::default(),
            }
        };

        let content;
        braced!(content in input);

        let entries: CheckEntries = content.parse()?;

        Ok(Self {
            check_providers,
            impl_generics,
            trait_name,
            context_type,
            where_clause,
            check_entries: entries,
        })
    }
}

impl Parse for CheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_entries: Punctuated<ParseCheckEntries, Comma> =
            Punctuated::parse_terminated(input)?;

        let entries = check_entries
            .into_iter()
            .flat_map(|check_entry| check_entry.entries.into_iter())
            .collect();

        Ok(Self { entries })
    }
}

impl Parse for ParseCheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_types: Vec<Type> = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
            Vec::from_iter(types)
        } else {
            let component_type: Type = input.parse()?;
            vec![component_type]
        };

        let component_params: Vec<Type> = if input.peek(Colon) {
            let _: Colon = input.parse()?;

            if input.peek(Bracket) {
                let content;
                bracketed!(content in input);

                let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
                types.into_iter().collect()
            } else {
                vec![input.parse()?]
            }
        } else {
            vec![]
        };

        let mut entries = Vec::new();

        let component_types_count = component_types.len();

        for component_type in component_types.iter() {
            if component_params.is_empty() {
                entries.push(CheckEntry {
                    component_type: component_type.clone(),
                    component_params: None,
                    span: component_type.span(),
                })
            } else {
                let component_params_count = component_params.len();

                for component_param in component_params.iter() {
                    let span = if component_types_count >= component_params_count {
                        component_type.span()
                    } else {
                        component_param.span()
                    };

                    entries.push(CheckEntry {
                        component_type: component_type.clone(),
                        component_params: Some(component_param.clone()),
                        span,
                    })
                }
            }
        }

        Ok(Self { entries })
    }
}
