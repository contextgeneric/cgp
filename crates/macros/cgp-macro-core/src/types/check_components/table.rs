use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Lt, Pound, Where};
use syn::{Attribute, Ident, Type, WhereClause, braced, parse2};

use crate::types::check_components::CheckEntries;
use crate::types::generics::ImplGenerics;
use crate::types::ident::IdentWithTypeArgs;

pub struct CheckComponentsTable {
    pub check_providers: Option<Punctuated<Type, Comma>>,
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub where_clause: WhereClause,
    pub check_entries: CheckEntries,
}

impl Parse for CheckComponentsTable {
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
            let context_type: IdentWithTypeArgs = parse2(context_type.to_token_stream())?;

            Ident::new(
                &format!("__Check{}", context_type.ident),
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
