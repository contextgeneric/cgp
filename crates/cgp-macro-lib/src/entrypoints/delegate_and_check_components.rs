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
    ImplGenerics,
};

pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateAndCheckSpec = parse2(body)?;

    let check_entries: Vec<CheckEntry> = spec
        .entries
        .iter()
        .flat_map(|entry| {
            entry.keys.iter().flat_map(|key| {
                let component_type = &key.component_type;
                let span = component_type.span();

                match &key.check_params {
                    Some(check_params) => check_params
                        .iter()
                        .map(|generic| CheckEntry {
                            component_type: component_type.clone(),
                            component_params: Some(generic.clone()),
                            span,
                        })
                        .collect::<Vec<_>>(),
                    None => vec![CheckEntry {
                        component_type: component_type.clone(),
                        component_params: None,
                        span,
                    }],
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
                .map(|key| DelegateKey {
                    ty: key.component_type,
                    generics: ImplGenerics::default(),
                })
                .collect();

            DelegateEntry {
                keys,
                value: entry.value,
                mode: entry.mode,
            }
        })
        .collect();

    let mut out =
        impl_delegate_components(&spec.context_type, &spec.impl_generics, &delegate_entries)?;

    let check_spec = CheckComponents {
        check_providers: None,
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
