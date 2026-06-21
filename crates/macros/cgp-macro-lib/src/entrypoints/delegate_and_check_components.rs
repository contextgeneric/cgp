use cgp_macro_core::types::check_components::{
    CheckComponentsTable, CheckEntries, CheckEntry, CheckKey, CheckValue, TypeWithGenerics,
};
use cgp_macro_core::types::generics::ImplGenerics;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Where};
use syn::{Type, WhereClause, parse2};

use crate::delegate_components::impl_delegate_components;
use crate::parse::{DelegateAndCheckSpec, DelegateEntry, DelegateKey};

pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateAndCheckSpec = parse2(body)?;

    let mut check_entries = Punctuated::new();

    for entry in &spec.entries {
        for key in &entry.keys {
            let component_type = &key.component_type;

            match &key.check_params {
                Some(check_params) => {
                    // Emit one check entry per param so that a single-key/single-param
                    // entry resolves the error span to the component type (via eval()'s
                    // `component_types_count >= component_params_count` heuristic), and so
                    // that an empty param list (i.e. `#[skip_check]`) emits no check at all.
                    for check_param in check_params {
                        check_entries.push(CheckEntry {
                            key: CheckKey::Single(component_type.clone()),
                            value: Some(CheckValue::Single(Box::new(TypeWithGenerics::from(
                                check_param.clone(),
                            )))),
                        });
                    }
                }
                None => {
                    check_entries.push(CheckEntry {
                        key: CheckKey::Single(component_type.clone()),
                        value: None,
                    });
                }
            }
        }
    }

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

    let check_spec = CheckComponentsTable {
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

    let items = check_spec.to_items()?;

    out.extend(quote! {
        #( #items )*
    });

    Ok(out)
}
