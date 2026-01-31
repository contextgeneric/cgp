use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Where};
use syn::{Type, WhereClause, parse2};

use crate::check_components::derive_check_components;
use crate::delegate_components::impl_delegate_components;
use crate::parse::{
    CheckComponents, CheckEntries, CheckEntry, DelegateAndCheckSpec, DelegateEntry, DelegateKey,
    DelegateValue, ImplGenerics,
};

pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateAndCheckSpec = parse2(body)?;

    let check_entries: Vec<CheckEntry> = spec
        .entries
        .iter()
        .flat_map(|entry| {
            entry.keys.iter().map(|component_type| {
                let span = component_type.span();

                CheckEntry {
                    component_type: component_type.clone(),
                    component_params: None,
                    span,
                }
            })
        })
        .collect();

    let delegate_entries: Punctuated<DelegateEntry<Type>, Comma> = spec
        .entries
        .into_iter()
        .map(|entry| {
            let keys = entry
                .keys
                .into_iter()
                .map(|ty| DelegateKey {
                    ty,
                    generics: ImplGenerics::default(),
                })
                .collect();

            let value = DelegateValue::Type(entry.value);

            DelegateEntry {
                keys,
                value,
                mode: entry.mode,
            }
        })
        .collect();

    let mut out =
        impl_delegate_components(&spec.provider_type, &spec.impl_generics, &delegate_entries)?;

    let check_spec = CheckComponents {
        check_provider: None,
        impl_generics: spec.impl_generics,
        trait_name: spec.trait_name,
        context_type: spec.context_type,
        where_clause: WhereClause {
            where_token: Where(Span::call_site()),
            predicates: Punctuated::default(),
        },
        check_entries: CheckEntries {
            entries: check_entries,
        },
    };

    let (check_item_trait, check_item_impls) = derive_check_components(&check_spec)?;

    out.extend(check_item_trait.to_token_stream());
    out.append_all(check_item_impls);

    Ok(out)
}
